Command-line options for cerberusctl
==================================

See `EXAMPLES.rst <EXAMPLES.rst>`_ for examples on how to use.

``cerberusctl`` is split into a number of subcommands based on currency and/or
functionality. The best way to get up-to-date help is to use the integrated help option.

The full list of subcommands can be seen like so:

.. code::

  cerberusctl --help

Each subcommand has its own help, shown with ``cerberusctl <command> --help``.

List of subcommands
-------------------

For convenience of search, the help texts of all commands and subcommands are listed
on one page here.

.. ### ALL CONTENT BELOW IS GENERATED BY helper-scripts/make-options-rst.py ###
.. code::

  Usage: cerberusctl [OPTIONS] COMMAND [ARGS]...

  Options:
    -p, --path TEXT           Select device by specific path.
    -v, --verbose             Show communication messages.
    -j, --json                Print result as JSON object
    -P, --passphrase-on-host  Enter passphrase on host.
    -S, --script              Use UI for usage in scripts.
    -s, --session-id HEX      Resume given session ID.
    -r, --record TEXT         Record screen changes into a specified directory.
    --version                 Show the version and exit.
    --help                    Show this message and exit.

  Commands:
    binance            Binance Chain commands.
    btc                Bitcoin and Bitcoin-like coins commands.
    cardano            Cardano commands.
    clear-session      Clear session (remove cached PIN, passphrase, etc.).
    cosi               CoSi (Cothority / collective signing) commands.
    crypto             Miscellaneous cryptography features.
    debug              Miscellaneous debug features.
    device             Device management commands - setup, recover seed, wipe, etc.
    eos                EOS commands.
    ethereum           Ethereum commands.
    fido               FIDO2, U2F and WebAuthN management commands.
    firmware           Firmware commands.
    get-features       Retrieve device features and settings.
    get-session        Get a session ID for subsequent commands.
    list               List connected Cerberus devices.
    monero             Monero commands.
    nem                NEM commands.
    ping               Send ping message.
    ripple             Ripple commands.
    set                Device settings.
    solana             Solana commands.
    stellar            Stellar commands.
    tezos              Tezos commands.
    usb-reset          Perform USB reset on stuck devices.
    version            Show version of cerberusctl/cerberuslib.
    wait-for-emulator  Wait until Cerberus Emulator comes up.

Binance Chain commands.
~~~~~~~~~~~~~~~~~~~~~~~

.. code::

  cerberusctl binance --help

.. code::

  Usage: cerberusctl binance [OPTIONS] COMMAND [ARGS]...

    Binance Chain commands.

  Options:
    --help  Show this message and exit.

  Commands:
    get-address     Get Binance address for specified path.
    get-public-key  Get Binance public key.
    sign-tx         Sign Binance transaction.

Bitcoin and Bitcoin-like coins commands.
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code::

  cerberusctl btc --help

.. code::

  Usage: cerberusctl btc [OPTIONS] COMMAND [ARGS]...

    Bitcoin and Bitcoin-like coins commands.

  Options:
    --help  Show this message and exit.

  Commands:
    get-address      Get address for specified path.
    get-descriptor   Get descriptor of given account.
    get-public-node  Get public node of given path.
    sign-message     Sign message using address of given path.
    sign-tx          Sign transaction.
    verify-message   Verify message.

Cardano commands.
~~~~~~~~~~~~~~~~~

.. code::

  cerberusctl cardano --help

.. code::

  Usage: cerberusctl cardano [OPTIONS] COMMAND [ARGS]...

    Cardano commands.

  Options:
    --help  Show this message and exit.

  Commands:
    get-address             Get Cardano address.
    get-native-script-hash  Get Cardano native script hash.
    get-public-key          Get Cardano public key.
    sign-tx                 Sign Cardano transaction.

CoSi (Cothority / collective signing) commands.
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code::

  cerberusctl cosi --help

.. code::

  Usage: cerberusctl cosi [OPTIONS] COMMAND [ARGS]...

    CoSi (Cothority / collective signing) commands.

  Options:
    --help  Show this message and exit.

  Commands:
    commit  Ask device to commit to CoSi signing.
    sign    Ask device to sign using CoSi.

Miscellaneous cryptography features.
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code::

  cerberusctl crypto --help

.. code::

  Usage: cerberusctl crypto [OPTIONS] COMMAND [ARGS]...

    Miscellaneous cryptography features.

  Options:
    --help  Show this message and exit.

  Commands:
    decrypt-keyvalue  Decrypt value by given key and path.
    encrypt-keyvalue  Encrypt value by given key and path.
    get-entropy       Get random bytes from device.

Miscellaneous debug features.
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code::

  cerberusctl debug --help

.. code::

  Usage: cerberusctl debug [OPTIONS] COMMAND [ARGS]...

    Miscellaneous debug features.

  Options:
    --help  Show this message and exit.

  Commands:
    record      Record screen changes into a specified directory.
    send-bytes  Send raw bytes to Cerberus.

Device management commands - setup, recover seed, wipe, etc.
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code::

  cerberusctl device --help

.. code::

  Usage: cerberusctl device [OPTIONS] COMMAND [ARGS]...

    Device management commands - setup, recover seed, wipe, etc.

  Options:
    --help  Show this message and exit.

  Commands:
    authenticate          Get information to verify the authenticity of the device.
    backup                Perform device seed backup.
    load                  Upload seed and custom configuration to the device.
    prodtest-t1           Perform a prodtest on Model One.
    reboot-to-bootloader  Reboot device into bootloader mode.
    recover               Start safe recovery workflow.
    sd-protect            Secure the device with SD card protection.
    set-busy              Show a "Do not disconnect" dialog.
    setup                 Perform device setup and generate new seed.
    tutorial              Show on-device tutorial.
    unlock-bootloader     Unlocks bootloader.
    wipe                  Reset device to factory defaults and remove all private data.

EOS commands.
~~~~~~~~~~~~~

.. code::

  cerberusctl eos --help

.. code::

  Usage: cerberusctl eos [OPTIONS] COMMAND [ARGS]...

    EOS commands.

  Options:
    --help  Show this message and exit.

  Commands:
    get-public-key    Get Eos public key in base58 encoding.
    sign-transaction  Sign EOS transaction.

Ethereum commands.
~~~~~~~~~~~~~~~~~~

.. code::

  cerberusctl ethereum --help

.. code::

  Usage: cerberusctl ethereum [OPTIONS] COMMAND [ARGS]...

    Ethereum commands.

    Most Ethereum commands now require the host to specify definition of a network and possibly an
    ERC-20 token. These definitions can be automatically fetched using the `-a` option.

    You can also specify a custom definition source using the `-d` option. Allowable values are:

    - HTTP or HTTPS URL
    - path to local directory
    - path to local tar archive
    

    For debugging purposes, it is possible to force use a specific network and token definition by
    using the `--network` and `--token` options. These options accept either a path to a file with a
    binary blob, or a hex-encoded string.

  Options:
    -d, --definitions TEXT  Source for Ethereum definition blobs.
    -a, --auto-definitions  Automatically download required definitions from cerberus.uraanai.com
    --network TEXT          Network definition blob.
    --token TEXT            Token definition blob.
    --help                  Show this message and exit.

  Commands:
    get-address           Get Ethereum address in hex encoding.
    get-public-node       Get Ethereum public node of given path.
    sign-message          Sign message with Ethereum address.
    sign-tx               Sign (and optionally publish) Ethereum transaction.
    sign-typed-data       Sign typed data (EIP-712) with Ethereum address.
    sign-typed-data-hash  Sign hash of typed data (EIP-712) with Ethereum address.
    verify-message        Verify message signed with Ethereum address.

FIDO2, U2F and WebAuthN management commands.
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code::

  cerberusctl fido --help

.. code::

  Usage: cerberusctl fido [OPTIONS] COMMAND [ARGS]...

    FIDO2, U2F and WebAuthN management commands.

  Options:
    --help  Show this message and exit.

  Commands:
    counter      Get or set the FIDO/U2F counter value.
    credentials  Manage FIDO2 resident credentials.

Firmware commands.
~~~~~~~~~~~~~~~~~~

.. code::

  cerberusctl firmware --help

.. code::

  Usage: cerberusctl firmware [OPTIONS] COMMAND [ARGS]...

    Firmware commands.

  Options:
    --help  Show this message and exit.

  Commands:
    download  Download and save the firmware image.
    get-hash  Get a hash of the installed firmware combined with the optional challenge.
    update    Upload new firmware to device.
    verify    Verify the integrity of the firmware data stored in a file.

Monero commands.
~~~~~~~~~~~~~~~~

.. code::

  cerberusctl monero --help

.. code::

  Usage: cerberusctl monero [OPTIONS] COMMAND [ARGS]...

    Monero commands.

  Options:
    --help  Show this message and exit.

  Commands:
    get-address    Get Monero address for specified path.
    get-watch-key  Get Monero watch key for specified path.

NEM commands.
~~~~~~~~~~~~~

.. code::

  cerberusctl nem --help

.. code::

  Usage: cerberusctl nem [OPTIONS] COMMAND [ARGS]...

    NEM commands.

  Options:
    --help  Show this message and exit.

  Commands:
    get-address  Get NEM address for specified path.
    sign-tx      Sign (and optionally broadcast) NEM transaction.

Ripple commands.
~~~~~~~~~~~~~~~~

.. code::

  cerberusctl ripple --help

.. code::

  Usage: cerberusctl ripple [OPTIONS] COMMAND [ARGS]...

    Ripple commands.

  Options:
    --help  Show this message and exit.

  Commands:
    get-address  Get Ripple address
    sign-tx      Sign Ripple transaction

Device settings.
~~~~~~~~~~~~~~~~

.. code::

  cerberusctl set --help

.. code::

  Usage: cerberusctl set [OPTIONS] COMMAND [ARGS]...

    Device settings.

  Options:
    --help  Show this message and exit.

  Commands:
    auto-lock-delay        Set auto-lock delay (in seconds).
    display-rotation       Set display rotation.
    experimental-features  Enable or disable experimental message types.
    flags                  Set device flags.
    homescreen             Set new homescreen.
    label                  Set new device label.
    language               Set new language with translations.
    passphrase             Enable, disable or configure passphrase protection.
    pin                    Set, change or remove PIN.
    safety-checks          Set safety check level.
    wipe-code              Set or remove the wipe code.

Solana commands.
~~~~~~~~~~~~~~~~

.. code::

  cerberusctl solana --help

.. code::

  Usage: cerberusctl solana [OPTIONS] COMMAND [ARGS]...

    Solana commands.

  Options:
    --help  Show this message and exit.

  Commands:
    get-address     Get Solana address.
    get-public-key  Get Solana public key.
    sign-tx         Sign Solana transaction.

Stellar commands.
~~~~~~~~~~~~~~~~~

.. code::

  cerberusctl stellar --help

.. code::

  Usage: cerberusctl stellar [OPTIONS] COMMAND [ARGS]...

    Stellar commands.

  Options:
    --help  Show this message and exit.

  Commands:
    get-address       Get Stellar public address.
    sign-transaction  Sign a base64-encoded transaction envelope.

Tezos commands.
~~~~~~~~~~~~~~~

.. code::

  cerberusctl tezos --help

.. code::

  Usage: cerberusctl tezos [OPTIONS] COMMAND [ARGS]...

    Tezos commands.

  Options:
    --help  Show this message and exit.

  Commands:
    get-address     Get Tezos address for specified path.
    get-public-key  Get Tezos public key.
    sign-tx         Sign Tezos transaction.

