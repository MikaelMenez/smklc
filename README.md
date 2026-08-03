# SMKL Transpiler

Um transpilador rápido, seguro e sem dependências escrito em Rust, projetado para converter a linguagem de marcação SMKL em HTML5 formatado e elegante.

---

## Proposta

O SMKL (Simple Markup Language) nasceu para resolver a verbosidade do HTML tradicional. Ele substitui a repetição de tags de fechamento (`</div>`, `</span>`) por uma hierarquia baseada em chaves `{}` e atributos entre parênteses `()`, inspirando-se em linguagens modernas de estilização e configuração.

### Por que usar o SMKL?
* Sem tags de fechamento ruidosas: O escopo dos elementos é determinado nativamente por blocos `{}`.
* Leitura rápida: Atributos ficam encapsulados e limpos no topo de cada nó.
* 100% Rust: Sem runtime pesado, sem dependências externas e com performance extrema.
* Saída Indentada: O HTML gerado é limpo, legível e automaticamente indentado.

---

## Funcionalidades

- Zero Dependências: Binário único, leve e portátil.
- Suporte a Void Tags: Sintaxe especial com barras verticais (`|meta...|`, `|img...|`, `|br|`) para elementos sem fechamento.
- Blocos Brutos (`style` e `script`): O parser preserva chaves e sintaxes internas do CSS e JavaScript sem quebrar o código.
- Comentários de Linha: Suporte a comentários iniciados por `//`.
- CLI Simples: Interface via linha de comando para rápida integração em pipelines de build.

---

## Guia de Sintaxe

# SMKL 

```smkl
html(lang="pt-BR") {
  head {
    |meta(charset="UTF-8")|
    title { "Página Exemplo" }
  }
  body {
    div(class="card", id="principal") {
      h1 { "SMKL Transpiler" }
      |hr|
      p { "Gerando HTML limpo a partir do Rust." }
      |img(src="preview.png", alt="Exemplo")|
    }
  }
}
```

# HTML Gerado
```html

<!DOCTYPE html>
<html lang="pt-BR">
  <head>
    <meta charset="UTF-8">
    <title>Página Exemplo</title>
  </head>
  <body>
    <div class="card" id="principal">
      <h1>SMKL Transpiler</h1>
      <hr>
      <p>Gerando HTML limpo a partir do Rust.</p>
      <img src="preview.png" alt="Exemplo">
    </div>
  </body>
</html>
``
