openssl genrsa -des3 -out ca.key 4096;

openssl req -x509 -new -nodes -key ca.key -subj "/C=IL/ST=TA/O=Carbon14/CN=ca" -sha256 -days 1024 -out ca.crt;

openssl genrsa -out node.key 2048;

openssl req -new -sha256 -key node.key -subj "/C=IL/ST=TA/O=Carbon14/CN=node" -out node.csr;

openssl x509 -req -extfile <(printf "subjectAltName=DNS:localhost,DNS:roach1,DNS:roach2,DNS:roach3") -in node.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out node.crt -days 500 -sha256;

openssl genrsa -out client.root.key 2048;

openssl req -new -sha256 -key client.root.key -subj "/C=IL/ST=TA/O=Carbon14/CN=root" -out client.root.csr;

openssl x509 -req -extfile <(printf "subjectAltName=DNS:localhost,DNS:root") -in client.root.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out client.root.crt -days 500 -sha256;
