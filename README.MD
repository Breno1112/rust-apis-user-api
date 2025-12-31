# 🚀 Rust User API — MongoDB + Redis + Performance Testing

Este repositório contém uma **API de usuários desenvolvida em Rust**, totalmente **containerizada**, que utiliza **MongoDB como banco de dados principal** e **Redis como camada de cache de alta performance**.

O projeto foi criado com dois objetivos centrais:

1. **Construir uma API moderna, performática e bem estruturada em Rust**
2. **Avaliar, na prática, o desempenho da linguagem Rust em cenários reais**, utilizando testes de carga com **Apache JMeter**

Mais do que uma simples API CRUD, este projeto funciona como um **laboratório técnico**, explorando arquitetura backend, cache, concorrência, performance e execução em ambientes containerizados.

---

## 📌 Visão Geral do Projeto

A aplicação expõe endpoints HTTP para gerenciamento de usuários e segue uma arquitetura simples, porém robusta:

- 🗄️ **MongoDB** como base de dados principal (persistência definitiva)
- ⚡ **Redis (single-node)** como cache em memória para leituras rápidas
- 🐳 **Docker / Docker Compose** para padronização do ambiente
- 📊 **Apache JMeter** para testes de carga e análise de performance
- 🦀 **Rust** como linguagem principal, focando em segurança, velocidade e baixo consumo de recursos

---

## 🎯 Objetivos do Projeto

### Objetivo principal

Desenvolver uma **API em Rust**, executando em containers, capaz de:

- Persistir dados de usuários de forma confiável
- Reduzir latência através de cache em memória
- Escalar operações de leitura com eficiência
- Ser facilmente testada sob carga elevada

### Objetivos secundários

- Avaliar o **desempenho do Rust** como backend web
- Comparar requisições **com e sem cache Redis**
- Medir latência, throughput e estabilidade
- Aplicar boas práticas de arquitetura backend
- Construir um projeto sólido para **portfólio profissional**

---

## 🧱 Arquitetura da Aplicação

A arquitetura segue o fluxo abaixo:

1. O cliente realiza uma requisição HTTP para a API
2. A API consulta o **Redis**:
   - ✅ Se o dado estiver em cache, a resposta é retornada imediatamente
   - ❌ Caso contrário:
     - A API consulta o **MongoDB**
     - Retorna o resultado ao cliente
     - Armazena o dado no Redis com **TTL de 5 minutos**
3. Requisições futuras são atendidas diretamente pelo cache

Esse padrão reduz o acesso ao banco de dados, melhora o tempo de resposta e aumenta a capacidade de atendimento da API.

---

## 🔧 Tecnologias Utilizadas

- **Rust**
  - Linguagem principal do projeto
  - Segurança de memória e alta performance
- **MongoDB**
  - Banco de dados NoSQL
  - Persistência principal dos usuários
- **Redis**
  - Cache em memória (single-node)
  - TTL configurado para 5 minutos
- **Docker & Docker Compose**
  - Padronização e facilidade de execução
- **Apache JMeter**
  - Testes de carga e estresse
  - Análise de performance

---

## 📡 Endpoints Disponíveis

A API disponibiliza endpoints REST para gerenciamento de usuários:

- `GET /health`  
  Health check da aplicação

- `GET /users/{id}`  
  Consulta de usuário (prioriza cache Redis)

- `POST /users`  
  Criação de usuário

- `PUT /users/{id}`  
  Atualização de usuário

- `DELETE /users/{id}`  
  Remoção de usuário

As operações de leitura utilizam cache para maximizar performance.

---

## 📊 Testes de Performance

Um dos focos centrais do projeto é a **avaliação de desempenho**.

Os testes foram realizados utilizando **Apache JMeter**, simulando múltiplos usuários simultâneos realizando requisições à API.

### Pontos avaliados

- Tempo médio de resposta
- Diferença entre:
  - Requisições com cache Redis
  - Requisições diretas ao MongoDB
- Estabilidade sob carga
- Comportamento da aplicação em ambiente containerizado

### Resultados observados

![](docs/cap1.png)

![](docs/cap2.png)

![](docs/cap3.png)

![](docs/cap4.png)

![](docs/cap5.png)

![](docs/cap6.png)

![](docs/cap7.png)

### 🧠 Análise geral dos resultados

- 🚀 Redução significativa de latência ao utilizar Redis
- 📈 Aumento do throughput em operações de leitura
- 🦀 Rust apresentou excelente estabilidade e baixo consumo de recursos
- ⚙️ A aplicação se manteve estável mesmo sob carga elevada

Esses resultados reforçam o uso do Rust como uma excelente escolha para APIs de alta performance.

---

## 🐳 Como Executar o Projeto

### Pré-requisitos

- Docker
- Docker Compose

### Execução

```bash
git clone https://github.com/Breno1112/rust-apis-user-api.git
cd rust-apis-user-api/localstack
docker compose up
```

Todos os serviços (API, MongoDB e Redis) serão iniciados automaticamente.

---

## 📁 Organização do Repositório

O repositório contém:
* Código-fonte da API
* Configurações de Docker e Docker Compose
* Arquivos e planos de teste do JMeter
* Documentação e exemplos

A estrutura foi pensada para facilitar leitura, manutenção e futuras extensões.

---

## 🧠 Aprendizados e Conclusões

Este projeto proporcionou aprendizados práticos importantes, como:

* Desenvolvimento de APIs web em Rust
* Implementação de cache em memória
* Arquitetura orientada à performance
* Testes de carga e análise de resultados
* Execução de aplicações modernas em containers

O projeto demonstra que Rust é uma alternativa sólida e madura para backends de alta performance, especialmente quando combinado com boas práticas arquiteturais.

---

## 👤 Autor

**Breno Rodrigo Fachini**

Software Enginner | AWS Certified

Este projeto faz parte do meu **portfólio pessoal**, com foco em arquitetura, performance e qualidade de código.