#include <stdio.h>

#include "loader.c"

#define abort(x) fprintf(stderr, x); return 1

size_t read_file(char *filename, char *contents, size_t nbytes);

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
