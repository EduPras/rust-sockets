<div align="center">
  
  # 🦀 rust-sockets 🧦
</div>

[![Rust](https://github.com/EduPras/rust-sockets/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/EduPras/rust-sockets/actions/workflows/rust.yml)

## Como executar

1. Baixe o [rustup](https://rustup.rs/)
2. Navegue até a pasta do seu projeto no terminal.

3. Compile e execute o código em um único comando:

    ```sh
    cargo run
    ```

## Descrição da Estrutura de Payloads

Todos os payloads começam com um prefixo `^` e terminam com um sufixo `$`. Os campos são sempre separados por um pipe `|`.

- Operação é um caractere único que define a ação a ser executada:
  - C: Create
  - R: Read
  - U: Update
  - D: Delete

- Todos os campos numéricos (Calorias, Carboidratos, Proteína e Gordura) são representados como números de ponto flutuante.
- Nome do Produto é uma string de texto.
- ID: Um hash MD5 do nome do produto. Este valor é gerado pelo cliente antes de enviar o payload e funciona como um identificador único para cada produto.
-----

### Payload de Requisição

O payload de requisição é utilizado para enviar comandos ao servidor.

* **Estrutura Completa**: Usada para operações de Create e Update

  ```
  ^Operação|Id|Nome do produto|Calorias|Carboidratos|Proteína|Gordura$
  ```

* **Estrutura Simplificada**: Usada para operações de Read e Delete

  ```
  ^Operação|Id$
  ```

**Exemplo de Requisição (Completa)**:
`^C|123|Barra de Proteína|200|25.3|20|5.2$`

-----

### Payload de Resposta

O payload de resposta é o retorno do servidor após o processamento de uma requisição. Ele informa o resultado da operação e, em alguns casos, os dados solicitados.

* **Estrutura Padrão**: Usada para a maioria das respostas, informando o status da operação.

  ```
  ^Operação|Status Code$
  ```

  * **Status Code**:
    * `200`: Sucesso.
    * `404`: Não encontrado (geralmente para operações de Read ou Delete com um ID inexistente).

* **Estrutura Completa (Resposta de Read)**: Usada somente para retornar os detalhes de um produto após uma requisição de Read bem-sucedida.

  ```
  ^Operação|Status Code|Id|Nome do produto|Calorias|Carboidratos|Proteína|Gordura$
  ```

**Exemplo de Resposta (Padrão - Sucesso)**:
`^C|200$`

**Exemplo de Resposta (Completa - Read)**:
`^R|200|123|Barra de Proteína|200|25|20|5$`