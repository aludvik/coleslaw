package main

import (
	"os"
	"fmt"
	"io/ioutil"
)

//go:generate ./gen.sh

func abort(msg string) {
	fmt.Fprintf(os.Stderr, msg)
	os.Exit(1)
}

func main() {
	var infile, passwd string

	if len(os.Args) == 2 {
		infile = os.Args[1]
	} else if len(os.Args) == 3 {
		infile = os.Args[1]
		passwd = os.Args[2]
	} else {
		abort("Invalid number of arguments\n")
	}

	buf, err := ioutil.ReadFile(infile)
	if err != nil {
		abort("Failed to read file\n")
	}

	pemstr := string(buf)
	pemstrLen := len(buf)

	priv, pub, err := loadPemKey(pemstr, pemstrLen, passwd)
	if err != nil {
		abort(fmt.Sprintf("Failed to load key: %v\n", err))
	}
	fmt.Println("Private Key:", priv)
	fmt.Println("Public	Key:", pub)
}
