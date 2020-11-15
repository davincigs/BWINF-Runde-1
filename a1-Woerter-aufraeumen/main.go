package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
	"unicode"
)

const samplePath = "beispieldaten/raetsel0.txt"

// Example represents an example test file
type Example struct {
	cloze string
	words string
}

type DestructuredExample struct {
	cloze      []string
	extensions []string
	qualities  []bool
	words      []string
}

func main() {
	example, err := readInSample(samplePath)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Eingabe:", example.cloze, example.words)

	fmt.Println(solve(example))
}

func readInSample(path string) (*Example, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	lines := []string{}
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if len(lines) != 2 {
		return nil, fmt.Errorf("file has %v lines, expected 2", len(lines))
	}

	return &Example{
		cloze: lines[0],
		words: lines[1],
	}, nil
}

func delChar(s []rune, index int) []rune {
	return append(s[0:index], s[index+1:]...)
}

func destructure(expl *Example) *DestructuredExample {
	cloze := strings.Split(expl.cloze, " ")
	words := strings.Split(expl.words, " ")
	ext := []string{}
	qual := []bool{}

	for i, gaps := range cloze {

		letters := 0

		for j, gap := range gaps {
			if unicode.IsLetter(rune(gap)) {
				cloze[i] = gaps
				letters++
				if len(gaps) == j+1 {
					ext = append(ext, "")
				}
			} else if gap == '_' {
				cloze[i] = gaps
				if len(gaps) == j+1 {
					ext = append(ext, "")
				}
			} else {
				ext = append(ext, string(gap))
				cloze[i] = string(delChar([]rune(gaps), j))
			}
		}

		if letters > 0 {
			qual = append(qual, true)
		} else {
			qual = append(qual, false)
		}
	}

	return &DestructuredExample{
		cloze: cloze, qualities: qual, extensions: ext, words: words,
	}
}

func solve(expl *Example) string {
	destr := destructure(expl)

	fmt.Println(destr.words, destr.cloze, destr.extensions, destr.qualities)

	result := []string{}

	for range destr.words {
		result = append(result, "")
	}

	for _, word := range destr.words {
		i := 0
		for _, char := range word {
			i++
			fmt.Print(string(char))
		}
		fmt.Print(i)
		fmt.Print(len(word))
		fmt.Println("")
	}

	for i, gaps := range destr.cloze {
		if destr.qualities[i] {
			for _, word := range destr.words {
				for j, gap := range gaps {
					for k, char := range word {
						if gap == char {
							fmt.Println(string(gap), string(char), len(gaps), len(word), gaps, word)
							if len(gaps) == len(word) {
								if j == k {
									result[i] = word
								}
							}
						}
					}
				}
			}
		}
	}

	/* for i, gaps := range destr.cloze {
		for _, word := range destr.words {
			for _, usedWord := range result {
				if word != usedWord {
					if len(gaps) == len(word) {
						result[i] = word
					}
				}
			}
		}
	} */

	return strings.Join(result, "")
}
