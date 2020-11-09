# Certificates

## Use the script `build-a-pki.sh` from rustls

## Instructions

### Generate a self-signed CA key and cert for the server

```
openssl genrsa -out server_ca.key
openssl req -x509 -new -key server_ca.key -out server_ca.cert -days 7300
```

### Generate a server key, server CSR, then sign it with server_ca
```
openssl genrsa -out server.key
openssl req -new -key server.key -out server.csr
openssl x509 -req -in server.csr -out server.cert -CAkey server_ca.key -CA server_ca.cert -days 7300 -CAcreateserial
```

### Inspect a cert
```
openssl x509 -in server.cert -text
```

### Inspect a CSR
```
openssl req -in server.csr -text
```

## openssl commands cheat sheet

Generate a key: `openssl genrsa`
Generate a self-signed certificate: `openssl req -x509 -new`
Generate a certificate signing request (CSR): `openssl req -new`
Sign a CSR: `openssl x509 -req`
