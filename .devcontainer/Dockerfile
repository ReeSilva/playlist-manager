FROM rust:bullseye

RUN echo "deb http://deb.debian.org/debian unstable main" >> /etc/apt/sources.list \
    && apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install curl git sudo gnupg2 \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*

RUN adduser --disabled-password --gecos '' reesilva
RUN adduser reesilva sudo
RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers
USER reesilva

# [Optional] Uncomment this section to install additional OS packages.
RUN sudo apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && sudo apt-get -y install --no-install-recommends libgtk-4-dev