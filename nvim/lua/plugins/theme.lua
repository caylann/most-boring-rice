return {
  {
    "fxn/vim-monochrome",
    lazy = false,
    priority = 1000,
    config = function()
      -- Переопределяем цвета до и после загрузки схемы
      vim.api.nvim_create_autocmd("ColorScheme", {
        pattern = "monochrome",
        callback = function()
          vim.api.nvim_set_hl(0, "Normal", { fg = "#FFFFFF", bg = "#000000" })
          vim.api.nvim_set_hl(0, "NormalNC", { fg = "#FFFFFF", bg = "#000000" })
          vim.api.nvim_set_hl(0, "Comment", { fg = "#666666", italic = true })
          vim.api.nvim_set_hl(0, "StatusLine", { fg = "#FFFFFF", bg = "#222222" })
          vim.api.nvim_set_hl(0, "LineNr", { fg = "#444444", bg = "#000000" })
          vim.api.nvim_set_hl(0, "CursorLineNr", { fg = "#FFFFFF", bg = "#111111" })

          -- Синхронизация с плагинами LazyVim
          vim.api.nvim_set_hl(0, "NeoTreeNormal", { fg = "#FFFFFF", bg = "#000000" })
          vim.api.nvim_set_hl(0, "NeoTreeNormalNC", { fg = "#FFFFFF", bg = "#000000" })
          vim.api.nvim_set_hl(0, "PmenuSel", { fg = "#000000", bg = "#FFFFFF", bold = true })
        end,
      })
      vim.cmd.colorscheme("monochrome")
    end,
  },
  {
    "LazyVim/LazyVim",
    opts = {
      colorscheme = "monochrome",
    },
  },
}
