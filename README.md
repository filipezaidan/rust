# Linguagem de Progamação - Rust

<p align="center">
<img src="https://www.freecodecamp.org/news/content/images/2021/01/rust-mascot.png" width="300">
</p>

Bem-vindo ao tuturial básico da linguagem Rust!

Nesta documentação, você as intruções básicas que precisa saber para começar a programar em Rust, desde a instalação, sintaxe básica até as características mais avançadas da linguagem. Esperamos que esta documentação ajude você a descobrir os benefícios e a elegância da programação em Rust.

---

## **_Tabela de conteúdo_**

1.  [Contexto Histórico](#contexto-histórico)
1.  [Características Gerais](#características-gerais)
1.  [Instalação](#instalação)
1.  [Sintaxe](#sintaxe)
    - [Declação de Variáveis](#declaração-de-variáveis)
    - [Como realizar um Hello World](#como-realizar-um-hello-world)
    - [Estruturas de Condição](#estruturas-de-condição)
      - [If/elseif/else](#ifelseifelse)
      - [Switch](#switch)
    - [Estruturas de Repetição](#estruturas-de-repetição)
      - [For](#for)
      - [While](#while)
    - [Modularização (Funções)](#modularização-funções)

---

## Contexto Histórico

A linguagem Rust foi criada em 2006 por Graydon Hoare, com o objetivo de fornecer uma alternativa segura e de alto desempenho às linguagens C e C++. A Mozilla Research passou a patrocinar o projeto em 2010, e a primeira versão estável foi lançada em 2015.

Desde então, Rust tem se tornado cada vez mais popular entre desenvolvedores que trabalham com aplicativos de sistema e de alto desempenho que exigem segurança e confiabilidade. Sua sintaxe clara e concisa, ênfase na segurança e desempenho, e comunidade ativa e acolhedora tornaram-na uma linguagem moderna em constante crescimento, frequentemente utilizada em jogos, sistemas operacionais e outras aplicações de alto desempenho.

## Características Gerais

Rust é uma linguagem de programação de sistema que se destaca por sua ênfase em segurança e desempenho. Sua sintaxe clara e concisa, gerenciamento de memória seguro, verificação de limites de vetor em tempo de compilação e recursos de programação concorrente segura são algumas de suas características mais distintas. Além disso, Rust é uma linguagem multiplataforma e tem uma comunidade ativa, acolhedora e pronta para ajudar.

## Instalação

### **_Instalação no Windows_**:

1.  Acesse o site oficial da Rust ([https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)) e clique no botão "Install Rust".
2.  Selecione o instalador para Windows e execute-o.
3.  Siga as instruções do instalador até que a instalação seja concluída.

### **_Instalação no Linux_**:

1.  Abra o terminal e atualize os pacotes do sistema com o comando "sudo apt-get update" (Ubuntu/Debian) ou "sudo yum update" (Red Hat/Fedora).
2.  Instale as dependências necessárias com o comando "sudo apt-get install build-essential" (Ubuntu/Debian) ou "sudo yum groupinstall 'Development Tools'" (Red Hat/Fedora).
3.  Acesse o site oficial da Rust ([https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)) e copie o comando de instalação da versão mais recente do Rust para Linux.
4.  Cole o comando no terminal e pressione Enter para iniciar o processo de instalação.
5.  Aguarde até que a instalação seja concluída.

Após concluir a instalação, você pode testar se a linguagem Rust está funcionando corretamente executando o comando "rustc --version" no terminal. Ele deve exibir a versão atual da linguagem instalada.

Caso não queira instalar na sua máquina, é possível executar codigo online utilizando o [playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021) do próprio rust.

## Sintaxe

### **_Declaração de variáveis_**

Em Rust, as variáveis são declaradas usando a palavra-chave "let" seguida pelo nome da variável e do valor atribuído a ela. A declaração de variáveis em Rust é do tipo estático, o que significa que o tipo da variável deve ser especificado no momento da declaração.

Aqui está um exemplo de como declarar variáveis em Rust:

```rust
let nome= "Filipe Zaidan";
let idade= 22;
let altura= 1.62;
```

Neste exemplo, três variáveis são declaradas: "nome", "idade" e "altura". A primeira variável armazena uma string, enquanto as outras duas armazenam números inteiros e decimais, respectivamente. Note que o tipo de dado de cada variável não é obrigatorio.

Rust é uma linguagem de programação fortemente tipada, o que significa que o tipo de dado deve ser especificado explicitamente. Além disso, as variáveis em Rust são sensíveis a maiúsculas e minúsculas, o que significa que "nome" e "Nome" são variáveis diferentes em Rust.

### **_Como realizar um Hello World_**

Para realizar um "Hello World" em Rust, podemos utilizar a função `println!`. Ela é usada para imprimir uma linha de texto no console e pode receber vários argumentos que serão impressos separados por vírgula.

```rust
fn main() {
    println!("Hello World");
}
```

Também podemos utilizar a macro "print!" que tem a mesma função da macro "println!" porém sem imprimir uma nova linha após a mensagem. O "print!" só pode receber um argumento por vez e retorna o valor 1 por padrão, o que o torna um pouco mais lento em relação ao "println!".

```rust
fn main() {
   print!("Hello World com print");
}
```

### **_Estruturas de Condição_**

Rust possui três estruturas de condição principais: `if`, `if-else`, e `if-elseif-else`.

A estrutura `if` é utilizada para testar uma condição simples, e executa um bloco de código se essa condição for verdadeira. Por exemplo:

### if:

```rust
fn main() {
	let x = 5;
	if x < 10 {
	    println!("x é menor do que 10");
	}
}
```

Se a condição `x < 10` for verdadeira, a mensagem "x é menor do que 10" será impressa no console.

### if-else:

A estrutura `if-else` é utilizada para testar uma condição e executar um bloco de código se a condição for verdadeira, e outro bloco se a condição for falsa. Por exemplo:

```rust
fn main() {
	let x = 5;
	if x < 10{
	    println!("x é menor do que 10");
	}else{
	    println!("x é maior ou igual a 10");
	}
}
```

Se a condição `x < 10` for verdadeira, a mensagem "x é menor do que 10" será impressa no console. Se a condição for falsa, a mensagem "x é maior ou igual a 10" será impressa.

### if-elseif-else:

A estrutura `if-else if-else` é utilizada para testar múltiplas condições e executar diferentes blocos de código baseados nesses testes. Por exemplo:

```rust
fn main() {
	let x = 5;
	if x < 0 {
	    println!("x é negativo");
	}else if x == 0 {
	    println!("x é zero");
	}else {
	    println!("x é positivo");
	}
}
```

Se `x` for menor que zero, a mensagem `x é negativo` será impressa. Casp `x` for igual a zero, a mensagem `x é zero` será impressa. Ou ainda `x` for maior do que zero, a mensagem `x é positivo` será impressa.

#### Switch

### **_Estruturas de Repetição_**

A estrutura de repetição no PHP permite que um conjunto de instruções seja executado várias vezes com base em uma condição específica. Existem duas estruturas de repetição principais no PHP: "while" e "for".

#### For

#### while

### **_Modularização (Funções)_**
