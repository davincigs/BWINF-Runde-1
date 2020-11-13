package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

const sampleFolder = "beispieldaten"
const numOfSampleFiles = 5

// Example represents an example test file
type Example struct {
	cloze string
	words string
}

func main() {
	examples := []*Example{}
	for i := 0; i < numOfSampleFiles; i++ {
		example, err := readInSample(fmt.Sprintf("%v/raetsel%v.txt", sampleFolder, i))
		if err != nil {
			log.Fatal(err)
			return
		}
		examples = append(examples, example)
	}

	for _, val := range examples {
		fmt.Println(val.cloze, val.words)
	}
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
