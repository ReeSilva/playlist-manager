FROM ubuntu

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install curl git sudo \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*

# RUN useradd -ms /bin/bash -p '' reesilva && echo "reesilva:reesilva" | chpasswd
# RUN mkdir -p /home/reesilva && chown -R reesilva:reesilva /home/reesilva

RUN adduser --disabled-password --gecos '' reesilva
RUN adduser reesilva sudo
RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers
USER reesilva

# [Optional] Uncomment this section to install additional OS packages.
RUN sudo apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && sudo apt-get -y install --no-install-recommends libgtk-4-dev \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN export PATH="$HOME/.cargo/bin:$PATH"