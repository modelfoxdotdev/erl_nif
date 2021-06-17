defmodule AddTest do
  use ExUnit.Case
  doctest Add

  test "adds two numbers" do
    assert Add.add(3, 4) == 7
  end
end
