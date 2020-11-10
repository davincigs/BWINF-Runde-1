package main

import "fmt"

func main() {

	/* content, err := ioutil.ReadFile("beispieldaten/spielstaerken1.txt")
	if err != nil {
		fmt.Println(err)
		return
	} */

	fmt.Println("Hallo, Theo!")

	player := [8]int{8, 10, 20, 30, 40, 50, 60, 100}
	ligaWinns := [8]int{0, 0, 0, 0, 0, 0, 0, 0}
	mostWinns := 0

	liga(player, ligaWinns, mostWinns)
}
func liga(player [8]int, ligaWinns [8]int, mostWinns int) {

	for i := 0; i < 8; i++ {
		for j := 0; player[i] < 8; j++ {
			if player[i] > player[j] {
				ligaWinns[i] = ligaWinns[i] + 1
			}
		}
	}
	for k := 0; k < 8; k++ {
		if ligaWinns[k] > ligaWinns[mostWinns] {
			mostWinns = k
		}
	}
	fmt.Println("Spieler Nr. ", mostWinns, " hat die meisten Siege in der Liga Variante.")
}
