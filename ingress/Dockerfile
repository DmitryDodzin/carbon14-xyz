FROM nginx:alpine

RUN rm -rf /etc/nginx/conf.d

COPY ./html/* /usr/share/nginx/html
COPY ./conf.d /etc/nginx/conf.d
COPY ./nginx.conf /etc/nginx/nginx.conf

EXPOSE 9999