# root.nvim

Easily find your project's root directory.

## Installation ##
Most of the source code is written in Rust so in order to build the plugin Rust needs to be installed on the system. 

Using [lazy.nvim](https://github.com/folke/lazy.nvim)

```lua
{
  "gmartsenkov/root.nvim",
  lazy = false,
  build = "make",
  config = function()
    require("root").setup {
        patterns = { ".git", "Gemfile" }
      }
  end
}
```

## Example Usage ##

```lua
  require('telescope.builtin').find_files({
    cwd = require("root").find() or vim.fn.expand('%:p:h')
  })
```
