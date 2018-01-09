# A gallocy-rs testing image.

FROM ubuntu
MAINTAINER Stephen Holsapple <sholsapp@gmail.com>
RUN mkdir -p /opt/app/bin
RUN mkdir -p /opt/app/etc
RUN mkdir -p /opt/app/var
ADD target/release/server /opt/app/bin/server
EXPOSE 8080
CMD ["/opt/app/bin/server"]
