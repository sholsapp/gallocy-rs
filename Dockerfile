# A gallocy-rs testing image.

FROM ubuntu
MAINTAINER Stephen Holsapple <sholsapp@gmail.com>
RUN mkdir /home/cthulhu/bin
RUN mkdir /home/cthulhu/etc
RUN mkdir /home/cthulhu/var
EXPOSE 8080
CMD /home/cthulhu/bin/server