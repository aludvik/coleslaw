#include <stdio.h>
#include <string.h>

#include <openssl/bio.h>
#include <openssl/evp.h>
#include <openssl/err.h>
#include <openssl/pem.h>
#include <openssl/engine.h>
#include <openssl/conf.h>

#define abort(x) fprintf(stderr, x); return 1
size_t read_file(char *filename, char *contents, size_t nbytes);

// Extract the private and public keys from the PEM file, using the supplied
// password to decrypt the file if encrypted. priv_key and pub_key must point to
// an array o at least 65 and 131 character respectively.
int load_pem_key(char *pemstr, size_t pemstr_len, char *password,
                 char *priv_key, char *pub_key);

int main(int argc, char **argv) {
  char *passwd = NULL;
  char *infile = NULL;

  char priv_key[65];
  char pub_key[131];

  char pemstr[512];
  size_t pemstr_len = 0;

  if (argc == 2) {
    infile = argv[1];
  } else if (argc == 3) {
    infile = argv[1];
    passwd = argv[2];
  } else {
    abort("Invalid number of arguments\n");
  }

  if ((pemstr_len = read_file(infile, pemstr, 1024)) <= 0) {
    abort("Failed to read file\n");
  }

  switch (load_pem_key(pemstr, pemstr_len, passwd, priv_key, pub_key)) {
    case -1:
      abort("Failed to decrypt or decode private key\n");
      break;
    case -2:
      abort("Failed to create new big number context\n");
      break;
    case -3:
      abort("Failed to load group\n");
      break;
    case -4:
      abort("Failed to load private key\n");
      break;
    case -5:
      abort("Failed to load public key point\n");
      break;
    case -6:
      abort("Failed to construct public key from point\n");
      break;
    default:
      printf("Private Key: %s\n", priv_key);
      printf("Public Key : %s\n", pub_key);
  }

  return 0;
}

size_t read_file(char *filename, char *contents, size_t nbytes) {
  // TODO: Error handling
  FILE *fin;
  size_t nread;
  fin = fopen(filename, "r");
  if (fin == NULL) {
    abort("Failed to open file\n");
  }
  nread = fread(contents, 1, nbytes, fin);
  fclose(fin);
  return nread;
}

int load_pem_key(char *pemstr, size_t pemstr_len, char *password,
                 char *out_priv_key, char *out_pub_key) {

  BIO *in = NULL;

  BN_CTX *ctx = NULL;
  const EC_GROUP *group;
  EC_KEY *eckey = NULL;
  const EC_POINT *pub_key_point = NULL;
  const BIGNUM *priv_key = NULL, *pub_key = NULL;

  char *priv_key_hex = NULL;
  char *pub_key_hex = NULL;

  in = BIO_new_mem_buf(pemstr, (int)pemstr_len);

  // Read key from stream, decrypting with password if not NULL
  if (password != NULL) {
    // Initialize ciphers
    ERR_load_crypto_strings ();
    OpenSSL_add_all_algorithms ();

    eckey = PEM_read_bio_ECPrivateKey(in, NULL, NULL, password);
    if (eckey == NULL) {
      return -1; // Failed to decrypt or decode private key
    }
  } else {
    if ((eckey = PEM_read_bio_ECPrivateKey(in, NULL, NULL, NULL)) == NULL) {
      return -1; // Failed to decode private key
    }
  }
  BIO_free(in);

  // Deconstruct key into big numbers
  if ((ctx = BN_CTX_new()) == NULL) {
    return -2; // Failed to create new big number context
  }
  if ((group = EC_KEY_get0_group(eckey)) == NULL) {
    return -3; // Failed to load group
  }
  if ((priv_key = EC_KEY_get0_private_key(eckey)) == NULL) {
    return -4; // Failed to load private key
  }
  if ((pub_key_point = EC_KEY_get0_public_key(eckey)) == NULL) {
    return -5; // Failed to load public key point
  }
  pub_key = EC_POINT_point2bn(group, pub_key_point, EC_KEY_get_conv_form(eckey), NULL, ctx);
  if (pub_key == NULL) {
    return -6; // Failed to construct public key from point
  }

  // TODO: Convert public key to point compression form
  priv_key_hex = BN_bn2hex(priv_key);
  pub_key_hex = BN_bn2hex(pub_key);
  strncpy(out_priv_key, priv_key_hex, 64 + 1);
  strncpy(out_pub_key, pub_key_hex, 130 + 1);
  OPENSSL_free(priv_key_hex);
  OPENSSL_free(pub_key_hex);
  return 0;
}
