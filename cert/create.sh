openssl genrsa -des3 -out ca.key 4096

openssl req -x509 -new -nodes -key ca.key -sha256 -days 1024 -out ca.crt

openssl genrsa -out node.key 2048

openssl req -new -sha256 -key node.key -subj "/C=IL/ST=TA/O=Carbon14/CN=node" -out node.csr

openssl x509 -req -extfile <(printf "subjectAltName=DNS:roach1,DNS:roach2,DNS:roach3") -in node.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out node.crt -days 500 -sha256