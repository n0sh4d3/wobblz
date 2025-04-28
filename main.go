package main

import (
	"bufio"
	"fmt"
	mini "github.com/mvstermind/mini-cli"
	"net/http"
	"os"
	"strings"
	"time"
)

func main() {
	t := "target"
	w := "wordlist"
	target := mini.NewArg(firstLett(t), t, "target URL to fuzz", true)
	wordlist := mini.NewArg(firstLett(w), w, "wordlist file path", true)
	cmds := mini.AddArguments(target, wordlist)
	args := cmds.Execute()

	targetVal := args["-t"]
	wlistVal := args["-w"]

	fileContent, err := readFile(wlistVal)
	if err != nil {
		fmt.Printf("Error reading wordlist: %v\n", err)
		os.Exit(1)
	}

	if err := fuzzyReplace(targetVal, "FUZZ", fileContent); err != nil {
		fmt.Printf("Error occurred: %v\n", err)
		os.Exit(1)
	}
}

func fuzzyReplace(target string, fuzzWord string, fileContent []string) error {
	if !strings.Contains(target, fuzzWord) {
		return fmt.Errorf("fuzz keyword '%s' not found in target URL", fuzzWord)
	}

	client := &http.Client{
		Timeout: 10 * time.Second,
	}

	for _, word := range fileContent {
		if word == "" {
			continue
		}

		url := strings.ReplaceAll(target, fuzzWord, word)

		resp, err := client.Get(url)
		if err != nil {
			fmt.Printf("Error requesting %s: %v\n", url, err)
			continue
		}

		fmt.Printf("%s -> %d\n", url, resp.StatusCode)
		resp.Body.Close()
	}

	return nil
}

func firstLett(word string) string {
	if len(word) == 0 {
		return ""
	}
	return string(word[0])
}

func readFile(filename string) ([]string, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var words []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		word := strings.TrimSpace(scanner.Text())
		if word != "" && !strings.HasPrefix(word, "#") { // Skip comments and empty lines
			words = append(words, word)
		}
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return words, nil
}
