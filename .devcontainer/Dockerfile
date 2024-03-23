FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye as base

# ARG USERNAME=user-name-goes-here
# ARG USER_UID=1000
# ARG USER_GID=$USER_UID

# RUN echo $USERNAME && echo ${USER_UID} && echo ${USER_GID}
# # Create the user
# RUN groupadd --gid $USER_GID $USERNAME \
#     && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
#     #
#     # [Optional] Add sudo support. Omit if you don't need to install software after connecting.
#     && apt-get update \
#     && apt-get install -y sudo \
#     && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
#     && chmod 0440 /etc/sudoers.d/$USERNAME

# ********************************************************
# * Anything else you want to do like clean up goes here *
# ********************************************************

# [Optional] Set the default user. Omit if you want to keep the default as root.

FROM base as dev
RUN apt-get update && apt-get install -y xz-utils libudev-dev

# RUN /bin/sh -c "ls -l ."
# COPY ./build-context/packages /tmp/
# RUN xargs -a /tmp/packages apt-get -y install --no-install-recommends \
#     && apt-get autoremove -y \
#     && apt-get clean \
#     && rm /tmp/packages
USER vscode
COPY Cargo.toml /tmp/
RUN awk -F' = ' '/ = /{print $2 " " $1}' /tmp/Cargo.toml | xargs -n2 -P$(nproc) cargo install --locked --version \
    && sudo rm -rf /tmp/Cargo.toml "$CARGO_HOME/registry"

# # Install the xz-utils package
# RUN apt-get update && apt-get install -y xz-utils libudev-dev
