# RustyTerm

Um emulador de terminal moderno e leve para Linux.

## Sobre

RustyTerm é um terminal simples e eficiente com suporte a múltiplas abas e temas de cores personalizáveis.

## Instalação

### Ubuntu/Debian

```bash
sudo dpkg -i rustyterm_0.1.0_amd64.deb
```

Após a instalação, o RustyTerm estará disponível no menu de aplicativos ou pode ser iniciado pelo terminal:

```bash
rustyterm
```

## Funcionalidades

- Múltiplas abas em uma única janela
- 4 temas de cores incluídos
- Configuração automática salva
- Títulos de aba dinâmicos
- Reordenação de abas com arrastar e soltar

## Como Usar

### Abas

- **Nova aba**: Clique no botão `+` ou pressione `Ctrl+Shift+T`
- **Fechar aba**: Clique no `X` da aba ou pressione `Ctrl+Shift+W`
- **Reordenar**: Arraste a aba para a posição desejada

### Temas

Clique no ícone de menu (canto superior direito) e selecione um tema:

- **default** - Tema escuro (Catppuccin)
- **light** - Tema claro
- **solarized_dark** - Solarized Dark
- **dracula** - Dracula

## Atalhos de Teclado

| Atalho | Ação |
|--------|------|
| `Ctrl+Shift+T` | Nova aba |
| `Ctrl+Shift+W` | Fechar aba atual |

## Configuração

As configurações são salvas automaticamente em `~/.config/rustyterm/config.toml`:

```toml
theme = "default"
font_family = "Monospace"
font_size = 12
scrollback_lines = 10000
window_width = 800
window_height = 600
```

Edite este arquivo para personalizar:

- **theme** - Nome do tema de cores
- **font_family** - Fonte do terminal
- **font_size** - Tamanho da fonte
- **scrollback_lines** - Quantidade de linhas no histórico
- **window_width/height** - Tamanho inicial da janela

## Licença

MIT
