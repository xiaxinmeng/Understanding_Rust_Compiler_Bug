elixir
defmodule MyModule do
  @doc """
  Given a number, returns true if the number is even and else otherwise.

  ## Example
    iex> MyModule.even?(2)
    true
    iex> MyModule.even?(3)
    false
  """
  def even?(number) do
    rem(number, 2) == 0
  end
end
