local M = {}

local options = {}
local defaults = {
  patterns = { ".git", "Gemfile" }
}

function M.setup(opts)
  options = vim.tbl_deep_extend("force", {}, defaults, opts or {})
end

function M.find()
  local file = vim.api.nvim_buf_get_name(0)
  return require("root_backend").find(file, options)
end

return M;
