package main

// #cgo LDFLAGS: -lcrypto
// #include <stdio.h>
// #include <string.h>
//
// #include <openssl/bio.h>
// #include <openssl/evp.h>
// #include <openssl/err.h>
// #include <openssl/pem.h>
// #include <openssl/engine.h>
// #include <openssl/conf.h>
//
// int load_pem_key(char *pemstr, size_t pemstr_len, char *password,
//									char *out_priv_key, char *out_pub_key) {
//
//	 BIO *in = NULL;
//
//	 BN_CTX *ctx = NULL;
//	 const EC_GROUP *group;
//	 EC_KEY *eckey = NULL;
//	 const EC_POINT *pub_key_point = NULL;
//	 const BIGNUM *priv_key = NULL, *pub_key = NULL;
//
//	 char *priv_key_hex = NULL;
//	 char *pub_key_hex = NULL;
//
//	 in = BIO_new_mem_buf(pemstr, (int)pemstr_len);
//
//	 // Read key from stream, decrypting with password if not NULL
//	 if (password != NULL && strcmp(password, "") != 0) {
//		 // Initialize ciphers
//		 ERR_load_crypto_strings ();
//		 OpenSSL_add_all_algorithms ();
//
//		 eckey = PEM_read_bio_ECPrivateKey(in, NULL, NULL, password);
//		 if (eckey == NULL) {
//			 return -1; // Failed to decrypt or decode private key
//		 }
//	 } else {
//		 if ((eckey = PEM_read_bio_ECPrivateKey(in, NULL, NULL, NULL)) == NULL) {
//			 return -1; // Failed to decode private key
//		 }
//	 }
//	 BIO_free(in);
//
//	 // Deconstruct key into big numbers
//	 if ((ctx = BN_CTX_new()) == NULL) {
//		 return -2; // Failed to create new big number context
//	 }
//	 if ((group = EC_KEY_get0_group(eckey)) == NULL) {
//		 return -3; // Failed to load group
//	 }
//	 if ((priv_key = EC_KEY_get0_private_key(eckey)) == NULL) {
//		 return -4; // Failed to load private key
//	 }
//	 if ((pub_key_point = EC_KEY_get0_public_key(eckey)) == NULL) {
//		 return -5; // Failed to load public key point
//	 }
//	 pub_key = EC_POINT_point2bn(group, pub_key_point, EC_KEY_get_conv_form(eckey), NULL, ctx);
//	 if (pub_key == NULL) {
//		 return -6; // Failed to construct public key from point
//	 }
//
//	 // TODO: Convert public key to point compression form
//	 priv_key_hex = BN_bn2hex(priv_key);
//	 pub_key_hex = BN_bn2hex(pub_key);
//	 strncpy(out_priv_key, priv_key_hex, 64 + 1);
//	 strncpy(out_pub_key, pub_key_hex, 130 + 1);
//	 OPENSSL_free(priv_key_hex);
//	 OPENSSL_free(pub_key_hex);
//	 return 0;
// }
import "C"

import (
	"os"
	"fmt"
	"io/ioutil"
)

func abort(msg string) {
	fmt.Fprintf(os.Stderr, msg)
	os.Exit(1)
}

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
