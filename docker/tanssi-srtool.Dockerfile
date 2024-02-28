ARG SRTOOL_IMAGE_TAG

FROM paritytech/srtool:${SRTOOL_IMAGE_TAG}

USER root

RUN apt-get update && \
    apt-get install openssh-server -y

RUN rustup component add rust-src
USER 1001