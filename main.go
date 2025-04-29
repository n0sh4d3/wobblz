package main

import (
	"bufio"
	"flag"
	"fmt"
	"io"
	"math/rand"
	"net/http"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"sync"
	"time"
)

const (
	ColorReset    = "\033[0m"
	ColorPink     = "\033[38;5;219m"
	ColorLavender = "\033[38;5;183m"
	ColorCyan     = "\033[38;5;123m"
	ColorYellow   = "\033[38;5;228m"
	ColorGreen    = "\033[38;5;158m"
	ColorRed      = "\033[38;5;211m"
	ColorBlue     = "\033[38;5;147m"
)

const (
	EmojiSparkles      = "✨"
	EmojiStar          = "🌟"
	EmojiHeartSparkles = "💖"
	EmojiPaw           = "🐾"
	EmojiCookie        = "🍪"
	EmojiStrawberry    = "🍓"
	EmojiSad           = "(｡•́︿•̀｡)"
	EmojiBlush         = "(´｡• ᵕ •｡`) ♡"
	EmojiHappy         = "(≧◡≦) ♡"
	EmojiFeral         = "(ง'̀-'́)ง"
	EmojiSpicy         = "😳💥"
)

type Config struct {
	Fuzzword  string            `json:"fuzzword"`
	Wordlist  string            `json:"wordlist"`
	Threads   int               `json:"threads"`
	Timeout   int               `json:"timeout"`
	UserAgent string            `json:"user_agent"`
	Headers   map[string]string `json:"headers"`
}

type Result struct {
	URL           string
	StatusCode    int
	ContentLength int64
	ResponseTime  time.Duration
	ThreadID      int
	SpicyMatch    bool
}

func printBanner() {
	banner := `
 🐧💖 ~ wobblz ~ 💖🐧  
🌸 smol babie fuzzer that waddlez into ur URLz 🥺💕

˚∧＿∧  　+        —̳͟͞͞💗
(  •‿• )つ  —̳͟͞͞ 💗         —̳͟͞͞💗
(つ　 <                —̳͟͞͞💗
｜　 _つ      +  —̳͟͞͞💗         —̳͟͞͞💗 ˚
\し´
`
	fmt.Printf("%s%s%s\n", ColorPink, banner, ColorReset)
}

func LoadConfig(path string) (Config, error) {
	var config Config

	// Check if file exists
	if _, err := os.Stat(path); os.IsNotExist(err) {
		// Return default config if file doesn't exist
		return Config{
			Fuzzword:  "FUZZ",
			Threads:   10,
			Timeout:   10,
			UserAgent: "wobblz/1.0 (◕‿◕✿)",
			Headers:   make(map[string]string),
		}, nil
	}

	// Read the file
	data, err := os.ReadFile(path)
	if err != nil {
		return config, err
	}

	// Parse the file (simple version for now)
	lines := strings.Split(string(data), "\n")
	for _, line := range lines {
		line = strings.TrimSpace(line)

		// Skip comments and empty lines
		if strings.HasPrefix(line, "//") || line == "" {
			continue
		}

		parts := strings.SplitN(line, "=", 2)
		if len(parts) != 2 {
			continue
		}

		key := strings.TrimSpace(parts[0])
		value := strings.TrimSpace(parts[1])

		switch key {
		case "fuzzword":
			config.Fuzzword = value
		case "wordlist":
			config.Wordlist = value
		case "threads":
			if threads, err := parseInt(value); err == nil {
				config.Threads = threads
			}
		case "timeout":
			if timeout, err := parseInt(value); err == nil {
				config.Timeout = timeout
			}
		case "user_agent":
			config.UserAgent = value
		}
	}

	return config, nil
}

func parseInt(s string) (int, error) {
	// Remove any non-digit characters
	re := regexp.MustCompile(`\D`)
	clean := re.ReplaceAllString(s, "")

	// Convert to int
	var value int
	_, err := fmt.Sscanf(clean, "%d", &value)
	return value, err
}

// Read wordlist file with kawaii error handling
func readWordlist(filename string) ([]string, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, fmt.Errorf("can't find wordlistie %s: %v", filename, err)
	}
	defer file.Close()

	var words []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		word := strings.TrimSpace(scanner.Text())
		if word != "" && !strings.HasPrefix(word, "#") {
			words = append(words, word)
		}
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return words, nil
}

// Main function
func main() {
	// Load config.huggies
	configFile := "config.huggies"
	config, err := LoadConfig(configFile)
	if err != nil {
		fmt.Printf("%s%s couldn't read config.huggies: %v%s\n",
			ColorPink, EmojiSad, err, ColorReset)
	}

	// Define command line flags
	targetPtr := flag.String("t", "", "target URL with fuzzwordz (required)")
	wordlist1Ptr := flag.String("w1", "", "primary wordlist for first fuzzword")
	wordlist2Ptr := flag.String("w2", "", "wordlist for second fuzzword")
	wordlist3Ptr := flag.String("w3", "", "wordlist for third fuzzword")
	threadsPtr := flag.Int("uwu", config.Threads, "how many wobblies at once")
	timeoutPtr := flag.Int("timeout", config.Timeout, "timeout in seconds")
	dryRunPtr := flag.Bool("dry", false, "just pretend (no requests)")
	verbosePtr := flag.Bool("nya", false, "extra meowy output")
	spicyPtr := flag.String("spicy", "200,201,204,301,302,307", "status codes that are spicy")

	// Custom usage message
	flag.Usage = func() {
		fmt.Fprintf(os.Stderr, "%s%s wobblz - smol babie fuzzer %s%s\n\n",
			ColorPink, EmojiSparkles, EmojiHeartSparkles, ColorReset)
		fmt.Fprintf(os.Stderr, "Usage: %s [options]\n\n", os.Args[0])
		fmt.Fprintf(os.Stderr, "Options:\n")
		flag.PrintDefaults()
		fmt.Fprintf(os.Stderr, "\nExample:\n")
		fmt.Fprintf(os.Stderr, "  %s -t \"http://FUZZSUB.local:FUZZPORT/FUZZDIR\" -w1 subbies.txt -w2 portsies.txt -w3 dirzies.txt\n",
			os.Args[0])
	}

	// Parse flags
	flag.Parse()

	// Print kawaii banner
	printBanner()

	// Validate required flags
	if *targetPtr == "" {
		fmt.Printf("%s%s pwease tell me where to wobble with -t flag%s\n",
			ColorPink, EmojiSad, ColorReset)
		flag.Usage()
		os.Exit(1)
	}

	// Identify all fuzzwords in target
	fuzzPattern := regexp.MustCompile(`FUZZ[A-Z]*`)
	fuzzwords := fuzzPattern.FindAllString(*targetPtr, -1)

	if len(fuzzwords) == 0 {
		fmt.Printf("%s%s no fuzzwordz found in target! add something like FUZZ or FUZZSUB%s\n",
			ColorPink, EmojiSad, ColorReset)
		os.Exit(1)
	}

	// Make sure wordlists are provided for each fuzzword
	wordlistPtrs := []*string{wordlist1Ptr, wordlist2Ptr, wordlist3Ptr}

	if *wordlist1Ptr == "" && len(fuzzwords) > 0 {
		fmt.Printf("%s%s need at least -w1 wordlist for wobbling!%s\n",
			ColorPink, EmojiSad, ColorReset)
		flag.Usage()
		os.Exit(1)
	}

	// Read wordlists
	wordlists := make([][]string, len(wordlistPtrs))

	for i, wlPtr := range wordlistPtrs {
		if i < len(fuzzwords) && *wlPtr != "" {
			fmt.Printf("%s%s loading wordlist for %s...%s\n",
				ColorLavender, EmojiSparkles, fuzzwords[i], ColorReset)

			wl, err := readWordlist(*wlPtr)
			if err != nil {
				fmt.Printf("%s%s couldn't read wordlist: %v%s\n",
					ColorPink, EmojiSad, err, ColorReset)
				os.Exit(1)
			}

			wordlists[i] = wl
			fmt.Printf("%s%s loaded %d wordz for %s %s%s\n",
				ColorGreen, EmojiPaw, len(wl), fuzzwords[i], EmojiHappy, ColorReset)
		}
	}

	// Parse spicy status codes
	spicyCodes := make(map[int]bool)
	for _, codeStr := range strings.Split(*spicyPtr, ",") {
		var code int
		if _, err := fmt.Sscanf(codeStr, "%d", &code); err == nil {
			spicyCodes[code] = true
		}
	}

	// Show wobble config
	fmt.Printf("%s%s wobblz will try these comboz:%s\n",
		ColorCyan, EmojiBlush, ColorReset)

	totalCombos := 1
	for i, wl := range wordlists {
		if i < len(fuzzwords) && len(wl) > 0 {
			fmt.Printf("  %s%-10s%s with %s%d%s wordz\n",
				ColorPink, fuzzwords[i], ColorReset,
				ColorYellow, len(wl), ColorReset)
			totalCombos *= len(wl)
		}
	}

	fmt.Printf("%s%s total comboz to try: %s%d%s\n\n",
		ColorLavender, EmojiSparkles,
		ColorYellow, totalCombos, ColorReset)

	// Start the fuzzing
	fmt.Printf("%s%s wobblz iz wobbling... pls wait...%s\n",
		ColorPink, EmojiSad, ColorReset)

	// If we're in dry run mode, just print a few example URLs
	if *dryRunPtr {
		fmt.Printf("%s%s just pretending mode activated!%s\n",
			ColorYellow, EmojiPaw, ColorReset)

		// Generate a few random combinations
		rand.Seed(time.Now().UnixNano())
		for i := 0; i < 5; i++ {
			url := *targetPtr
			for j, fuzzword := range fuzzwords {
				if j < len(wordlists) && len(wordlists[j]) > 0 {
					randomWord := wordlists[j][rand.Intn(len(wordlists[j]))]
					url = strings.Replace(url, fuzzword, randomWord, 1)
				}
			}
			fmt.Printf("%s%s would try: %s%s\n",
				ColorCyan, EmojiSparkles, url, ColorReset)
		}

		fmt.Printf("\n%s%s and %d more comboz...%s\n",
			ColorYellow, EmojiPaw, totalCombos-5, ColorReset)

		os.Exit(0)
	}

	// Start fuzzing with multiple wordlists
	results := make(chan Result)
	var wg sync.WaitGroup

	// Result processing goroutine
	go func() {
		spicyCount := 0
		totalCount := 0

		for result := range results {
			totalCount++

			// Check if result is "spicy"
			isSpicy := spicyCodes[result.StatusCode]

			if isSpicy {
				spicyCount++
				fmt.Printf("%s%s OMG!! found sth spicy %s %s -> %d (%d bytes)%s\n",
					ColorYellow, EmojiStar, EmojiSpicy,
					result.URL, result.StatusCode, result.ContentLength,
					ColorReset)
			} else if *verbosePtr {
				fmt.Printf("%s%s trying %s... (%d)%s\n",
					ColorLavender, EmojiSparkles,
					result.URL, result.StatusCode,
					ColorReset)
			}

			// Print progress occasionally
			if totalCount%100 == 0 || totalCount == totalCombos {
				fmt.Printf("%s%s wobbled %d/%d comboz... found %d spicy bitz so far!%s\n",
					ColorCyan, EmojiPaw,
					totalCount, totalCombos, spicyCount,
					ColorReset)
			}
		}

		// Final summary
		fmt.Printf("\n%s%s all done! wobbled %d comboz, found %d spicy bitz! %s%s\n",
			ColorPink, EmojiSparkles,
			totalCount, spicyCount,
			EmojiHeartSparkles, ColorReset)

		// ASCII celebration if found something spicy
		if spicyCount > 0 {
			cookieArt := `
   🍪 < cookie for being a good wobblz!
˚∧＿∧  　     
(  •ᴗ• )     
(つ　 >       
｜　 _つ       
\し´
`
			fmt.Printf("%s%s%s", ColorYellow, cookieArt, ColorReset)
		}
	}()

	// Generate all combinations
	startTime := time.Now()

	// Create a client with timeout
	client := &http.Client{
		Timeout: time.Duration(*timeoutPtr) * time.Second,
		CheckRedirect: func(req *http.Request, via []*http.Request) error {
			return http.ErrUseLastResponse // Don't follow redirects
		},
	}

	// Semaphore for limiting concurrency
	sem := make(chan bool, *threadsPtr)

	// Function to recursively generate combinations
	var generateCombinations func(url string, depth int)
	generateCombinations = func(url string, depth int) {
		// Base case: all fuzzwords have been replaced
		if depth >= len(fuzzwords) {
			wg.Add(1)
			go func(finalURL string) {
				defer wg.Done()

				// Acquire semaphore
				sem <- true
				defer func() { <-sem }()

				// Make the request
				req, err := http.NewRequest("GET", finalURL, nil)
				if err != nil {
					if *verbosePtr {
						fmt.Printf("%s%s invalid URL: %s%s\n",
							ColorRed, EmojiSad, finalURL, ColorReset)
					}
					return
				}

				// Set a cute user agent
				req.Header.Set("User-Agent", config.UserAgent)

				resp, err := client.Do(req)
				if err != nil {
					if *verbosePtr {
						fmt.Printf("%s%s error requesting %s: %v%s\n",
							ColorRed, EmojiSad, finalURL, err, ColorReset)
					}
					return
				}
				defer resp.Body.Close()

				// Create result
				result := Result{
					URL:           finalURL,
					StatusCode:    resp.StatusCode,
					ContentLength: resp.ContentLength,
					ResponseTime:  time.Since(startTime),
				}

				// Also classify as spicy if body contains interesting keywords
				if resp.StatusCode < 400 {
					body, _ := io.ReadAll(resp.Body)
					lowerBody := strings.ToLower(string(body))

					spicyWords := []string{"admin", "password", "token", "secret", "api", "key", "private"}
					for _, word := range spicyWords {
						if strings.Contains(lowerBody, word) {
							result.SpicyMatch = true
							break
						}
					}
				}

				results <- result
			}(url)
			return
		}

		if depth >= len(wordlists) || len(wordlists[depth]) == 0 {
			generateCombinations(url, depth+1)
			return
		}

		for _, word := range wordlists[depth] {
			newURL := strings.Replace(url, fuzzwords[depth], word, 1)
			generateCombinations(newURL, depth+1)
		}
	}

	generateCombinations(*targetPtr, 0)

	wg.Wait()
	close(results)

	time.Sleep(100 * time.Millisecond)
}

func homeDir() string {
	home, err := os.UserHomeDir()
	if err != nil {
		return ""
	}
	return home
}

func expandPath(path string) string {
	if strings.HasPrefix(path, "~/") {
		return filepath.Join(homeDir(), path[2:])
	}
	return path
}
