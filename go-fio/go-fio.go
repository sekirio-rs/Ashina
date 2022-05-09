package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sync"
	"time"
)

const BENCH_SIZE int = 1024 * 32
const MAX_FILE int = 512
const BUFFER_LEN int = 1024

func fio(wg *sync.WaitGroup) {
	defer wg.Done()

	file, err := os.Open("../LICENSE")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	reader := bufio.NewReader(file)

	buf := make([]byte, BUFFER_LEN)

	_, err = reader.Read(buf)
	if err != nil {
		log.Fatal(err)
	}

	dev_null, err := os.Create("/dev/null")
	if err != nil {
		log.Fatal(err)
	}
	defer dev_null.Close()

	writer := bufio.NewWriter(dev_null)

	_, err = writer.Write(buf)
	if err != nil {
		log.Fatal(err)
	}
}

func main() {
	start := time.Now()

	for i := 0; i < BENCH_SIZE/MAX_FILE; i++ {
		var wg sync.WaitGroup

		for j := 0; j < MAX_FILE; j++ {
			wg.Add(1)

			go fio(&wg)
		}

		wg.Wait()
	}

	cost := time.Since(start)

	fmt.Printf("[go-fio] bench_size: %d, cost: %d micros\n", BENCH_SIZE, cost.Microseconds())
}
