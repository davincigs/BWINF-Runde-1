package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

const samplePath = "beispieldaten/raetsel0.txt"

// Example represents an example test file
type Example struct {
	cloze string
	words string
}

func main() {
	example, err := readInSample(samplePath)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(example.cloze, example.words)
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
