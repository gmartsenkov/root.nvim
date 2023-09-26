local M = {}

local options = {}
local defaults = {
  patterns = { ".git", "Gemfile" }
}

function M.setup(opts)
  options = vim.tbl_deep_extend("force", {}, defaults, opts or {})
end

function M.find(file)
  local lookup_file = file or vim.api.nvim_buf_get_name(0)
  return require("root_backend").find(lookup_file, options)
end

return M;
