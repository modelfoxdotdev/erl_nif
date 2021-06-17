defmodule Add do
  @on_load {:init, 0}
  def init do
    path = :filename.join(:code.priv_dir(:add), "libadd")
    :ok = :erlang.load_nif(path, nil)
  end

  def add(_, _) do
    :erlang.nif_error(:nif_not_loaded)
  end
end
