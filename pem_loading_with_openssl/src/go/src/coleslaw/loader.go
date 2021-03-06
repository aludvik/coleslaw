package main

// #cgo LDFLAGS: -lcrypto
// #include "../../../c/loader.c"
import "C"

import "fmt"

func loadPemKey(pemstr string, pemstrLen int, password string) (priv_key string, pub_key string, err error) {
	cPemstr := C.CString(pemstr)
	cPemstrLen := C.size_t(pemstrLen)
	cPassword := C.CString(password)
	cOutPrivKey := C.CString("-----------------------------------------------------------------")
	cOutPubKey := C.CString("-----------------------------------------------------------------------------------------------------------------------------------")
	errnum := C.load_pem_key(cPemstr, cPemstrLen, cPassword, cOutPrivKey, cOutPubKey)
	if errnum < 0 {
		var errstr string
		switch errnum {
		case -1:
			errstr = "Failed to decrypt or decode private key"
		case -2:
			errstr = "Failed to create new big number context"
		case -3:
			errstr = "Failed to load group"
		case -4:
			errstr = "Failed to load private key"
		case -5:
			errstr = "Failed to load public key point"
		case -6:
			errstr = "Failed to construct public key from point"
		}
		return "", "", fmt.Errorf(errstr)
	}
	return C.GoString(cOutPrivKey), C.GoString(cOutPubKey), nil
}
