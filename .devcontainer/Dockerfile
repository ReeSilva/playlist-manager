FROM reesilva/vscode-env:rust-bullseye

USER root

RUN echo "deb http://deb.debian.org/debian unstable main" >> /etc/apt/sources.list \
    && apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install curl git gnupg2 \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*

USER reesilva

# [Optional] Uncomment this section to install additional OS packages.
RUN rustup component add clippy
RUN sudo apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && sudo apt-get -y install --no-install-recommends libgtk-4-dev