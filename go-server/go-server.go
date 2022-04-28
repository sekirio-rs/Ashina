package main

import (
	"bufio"
	"fmt"
	"log"
	"net"
)

func serve(conn net.Conn) {
	defer conn.Close()

	reader := bufio.NewReader(conn)
	var buffer []byte

	_, err := reader.Read(buffer)
	if err != nil {
		log.Fatal(err)
	}

	writer := bufio.NewWriter(conn)
	resp := fmt.Sprintf("HTTP/1.1 200 Ok\r\nServer: Ashina\r\n\r\n%s", buffer)

	_, err = writer.WriteString(resp)
	if err != nil {
		log.Fatal(err)
	}

}

func main() {
	listener, err := net.Listen("tcp", "0.0.0.0:3344")
	if err != nil {
		log.Fatal(err)
	}
	defer listener.Close()

	for {
		conn, err := listener.Accept()
		if err != nil {
			log.Fatal(err)
		}

		go serve(conn)
	}
}
