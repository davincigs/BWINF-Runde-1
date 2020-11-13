package main

import (
	"fmt"
	"math/rand"
	"time"
)

func main() {
	rand.Seed(time.Now().UnixNano())
	/* content, err := ioutil.ReadFile("beispieldaten/spielstaerken1.txt")
	if err != nil {
		fmt.Println(err)
		return
	} */
	player := []int{8, 10, 20, 30, 40, 50, 60, 100}

	//liga(player)
	ko(player)
	//kox5()
}

func liga(player []int) {

	ligaWinns := []int{0, 0, 0, 0, 0, 0, 0, 0}
	mostLigaWinns := 0
	min := 0
	max := 0

	// durch alle Spieler loopen:
	for i := 0; i < 8; i++ {
		for j := 0; j < 8; j++ {
			// Sieger entscheiden:
			min = 1
			max = player[j] + player[i]
			v := rand.Intn(max-min) + min
			if v > player[j] {
				ligaWinns[i] = ligaWinns[i] + 1
			}
		}
	}
	// insgesamte Sieger ausgeben:
	for i := 0; i < 8; i++ {
		if ligaWinns[i] > ligaWinns[mostLigaWinns] {
			mostLigaWinns = i
		}
	}
	fmt.Println("Spieler Nr.", mostLigaWinns, "hat die meisten Siege in der Liga Variante.")
}

func ko(player []int) {
	group := [][]int{{}, {}}
	min := 1
	max := len(player)
	v := 0
	unique := false

	//einteilung der Spieler in Paare:
	for i := 0; i < len(player)/2; i++ {
		for unique == false {
			unique = true
			v = rand.Intn(max-min) + min
			group[i] = append(group[i], v)

			for j := 0; j < i; j++ {
				for k := 0; k < 2; k++ {
					for v == group[j][k] {
						unique = false
					}
				}
			}
		}
	}
}
func kox5() {

}

/* Raum für Notizen:
- Array größe 8 ersetzten...
-
-
*/
