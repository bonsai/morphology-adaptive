FROM python:3.11

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    build-essential \
    pkg-config \
    libssl-dev \
    fonts-liberation \
    libasound2 \
    libatk-bridge2.0-0 \
    libatk1.0-0 \
    libc6 \
    libcairo2 \
    libcups2 \
    libdbus-1-3 \
    libexpat1 \
    libfontconfig1 \
    libgbm1 \
    libgcc1 \
    libglib2.0-0 \
    libgtk-3-0 \
    libnspr4 \
    libnss3 \
    libpango-1.0-0 \
    libpangocairo-1.0-0 \
    libstdc++6 \
    libx11-6 \
    libx11-xcb1 \
    libxcb1 \
    libxcomposite1 \
    libxcursor1 \
    libxdamage1 \
    libxext6 \
    libxfixes3 \
    libxi6 \
    libxrandr2 \
    libxrender1 \
    libxss1 \
    libxtst6 \
    lsb-release \
    wget \
    xdg-utils \
    chromium && \
    curl -fsSL https://deb.nodesource.com/setup_22.x | bash - && \
    apt-get install -y nodejs && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install Rust and wasm-pack
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh


ENV PUPPETEER_SKIP_CHROMIUM_DOWNLOAD=true
ENV PUPPETEER_EXECUTABLE_PATH=/usr/bin/chromium

# Install git for algovivo installation
RUN apt-get update && apt-get install -y git

RUN pip install uv

RUN uv pip install --system torch==2.7.0

ARG MORPHOLOGY_ADAPTIVE_DIRNAME=/morphology-adaptive
ARG ALGOVIVO_REPO_DIRNAME=/opt/algovivo.repo

RUN mkdir ${MORPHOLOGY_ADAPTIVE_DIRNAME}
COPY ./algovivo.json ${MORPHOLOGY_ADAPTIVE_DIRNAME}/algovivo.json
COPY ./python ${MORPHOLOGY_ADAPTIVE_DIRNAME}/python

RUN python ${MORPHOLOGY_ADAPTIVE_DIRNAME}/python/scripts/install_algovivo.py \
   --system \
   --algovivo-config-filename ${MORPHOLOGY_ADAPTIVE_DIRNAME}/algovivo.json \
   --repo-dirname ${ALGOVIVO_REPO_DIRNAME}

ENV PYTHONPATH=${MORPHOLOGY_ADAPTIVE_DIRNAME}/python:${MORPHOLOGY_ADAPTIVE_DIRNAME}
ENV ALGOVIVO_NATIVE_LIB_FILENAME=${ALGOVIVO_REPO_DIRNAME}/build/native/algovivo.so

COPY ./data ${MORPHOLOGY_ADAPTIVE_DIRNAME}/data

COPY ./requirements.txt ${MORPHOLOGY_ADAPTIVE_DIRNAME}/requirements.txt
RUN pip install -r ${MORPHOLOGY_ADAPTIVE_DIRNAME}/requirements.txt

COPY ./game_logic.py ${MORPHOLOGY_ADAPTIVE_DIRNAME}/game_logic.py
COPY ./main.py ${MORPHOLOGY_ADAPTIVE_DIRNAME}/main.py
COPY ./index.html ${MORPHOLOGY_ADAPTIVE_DIRNAME}/index.html
COPY ./script.js ${MORPHOLOGY_ADAPTIVE_DIRNAME}/script.js
COPY rust ${MORPHOLOGY_ADAPTIVE_DIRNAME}/src
COPY ./Cargo.toml ${MORPHOLOGY_ADAPTIVE_DIRNAME}/Cargo.toml
COPY ./Cargo.lock ${MORPHOLOGY_ADAPTIVE_DIRNAME}/Cargo.lock

WORKDIR ${MORPHOLOGY_ADAPTIVE_DIRNAME}
RUN wasm-pack build --target web
EXPOSE 8000
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
# COPY ./attn ${MORPHOLOGY_ADAPTIVE_DIRNAME}/attn

# RUN npm ci --prefix ${ALGOVIVO_REPO_DIRNAME}