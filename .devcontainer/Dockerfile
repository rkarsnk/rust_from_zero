FROM --platform=linux/amd64 debian:bookworm
LABEL mainiainer="rkarsnk"
LABEL version="1.0"

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive

# Update and install basic dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    ca-certificates \
    git \
    vim \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Set default shell to bash
SHELL ["/bin/bash", "-c"]

# Default command
CMD ["bash"]