import main


def test_map_stellar_error_timeout():
    msg = main.map_stellar_error(TimeoutError("rpc hung"))
    assert "timeout" in msg


def test_tx_builders_exist():
    assert callable(main.build_deposit_tx)
    assert callable(main.build_execute_payment_tx)
