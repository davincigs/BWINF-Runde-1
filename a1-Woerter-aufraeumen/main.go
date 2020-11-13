package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

const sampleFolder = "beispieldaten"

// Example represents an example test file
type Example struct {
	cloze string
	words string
}

func main() {
	examples := []*Example{}
	for i := 0; i < 4; i++ {
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
	expl := new(Example)
	file, err := os.Open(path)
	if err != nil {
		return expl, err
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	line := 0
	for scanner.Scan() {
		if line == 0 {
			expl.cloze = scanner.Text()
		} else if line == 1 {
			expl.words = scanner.Text()
		}
		line++
	}

	return expl, nil
}
