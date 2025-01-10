FROM espressif/idf-rust:esp32s3_latest

USER root

RUN usermod -a -G dialout esp

USER esp

CMD [ "/bin/bash" ]