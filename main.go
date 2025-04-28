package main

import (
	"fmt"
	"strings"

	mini "github.com/mvstermind/mini-cli"
)

func main() {

	target := mini.NewArg("t", "target", "pewson to attack", true)

	cmds := mini.AddArguments(target)

	args := cmds.Execute()

	targteVal := args["-t"]

	if err := parseTarget(targteVal, "WUW"); err != nil {
		fmt.Println("ewwow occured: ", err)
	}
}

func parseTarget(target string, fuzzWord string) error {
	var (
		foundFirstChar bool
		targetSliced   []string
	)

	for i, c := range target {
		// fuck em runezz
		char := string(c)

		if char == string(fuzzWord[0]) && !foundFirstChar {
			foundFirstChar = true
			targetSliced = append(targetSliced, target[:i])
			targetSliced = append(targetSliced, target[i+len(fuzzWord):])

		}

	}
	fmt.Println(targetSliced)

	word := insertWord(targetSliced, "rizzla")
	println(word)

	return nil

}

func enumerate(sliced []string, wordlist []string) (int, error) {
	return 0, nil

}

func insertWord(sliced []string, word string) string {
	var fuzzedWord string

	for i := range sliced {
		fuzzedWord += sliced[i] + word

	}

	return strings.Trim(fuzzedWord, word)
}
