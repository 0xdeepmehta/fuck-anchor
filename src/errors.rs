use phf::phf_map;
pub static ANCHOR_PROGRAM: phf::Map<&'static str, &'static str> = phf_map! {
    // Instructions.
    "100" => "InstructionMissing: 8 byte instruction identifier not provided",
    "101" =>"InstructionFallbackNotFound: Fallback functions are not supported",
    "102" => "InstructionDidNotDeserialize: The program could not deserialize the given instruction",
    "103" => "InstructionDidNotSerialize: The program could not serialize the given instruction",

    // IDL instructions.
    "1000" => "IdlInstructionStub: The program was compiled without idl instructions",
    "1001"=> "IdlInstructionInvalidProgram: Invalid program given to the IDL instruction",

    // Constraints.
    "2000" => "ConstraintMut: A mut constraint was violated",
    "2001" => "ConstraintHasOne: A has one constraint was violated",
    "2002" => "ConstraintSigner: A signer constraint as violated",
    "2003" => "ConstraintRaw: A raw constraint was violated",
    "2004" => "ConstraintOwner: An owner constraint was violated",
    "2005" => "ConstraintRentExempt: A rent exemption constraint was violated",
    "2006" => "ConstraintSeeds: A seeds constraint was violated",
    "2007" => "ConstraintExecutable: An executable constraint was violated",
    "2008" => "ConstraintState: A state constraint was violated",
    "2009" => "ConstraintAssociated: An associated constraint was violated",
    "2010" => "ConstraintAssociatedInit: An associated init constraint was violated",
    "2011" => "ConstraintClose: A close constraint was violated",
    "2012" => "ConstraintAddress: An address constraint was violated",
    "2013" => "ConstraintZero: Expected zero account discriminant",
    "2014" => "ConstraintTokenMint: A token mint constraint was violated",
    "2015" => "ConstraintTokenOwner: A token owner constraint was violated",
    "2016" => "ConstraintMintMintAuthority: A mint mint authority constraint was violated",
    "2017" => "ConstraintMintFreezeAuthority: A mint freeze authority constraint was violated",
    "2018" => "ConstraintMintDecimals: A mint decimals constraint was violated",
    "2019" => "ConstraintSpace: A space constraint was violated",

    // Accounts.
    "3000" => "AccountDiscriminatorAlreadySet: The account discriminator was already set on this account",
    "3001" => "AccountDiscriminatorNotFound: No 8 byte discriminator was found on the account",
    "3002" => "AccountDiscriminatorMismatch: 8 byte discriminator did not match what was expected",
    "3003" => "AccountDidNotDeserialize: Failed to deserialize the account",
    "3004" => "AccountDidNotSerialize: Failed to serialize the account",
    "3005" => "AccountNotEnoughKeys: Not enough account keys given to the instruction",
    "3006" => "AccountNotMutable: The given account is not mutable",
    "3007" => "AccountNotProgramOwned: The given account is not owned by the executing program",
    "3008" => "InvalidProgramId: Program ID was not as expected",
    "3009" => "InvalidProgramExecutable: Program account is not executable",
    "3010" => "AccountNotSigner: The given account did not sign",
    "3011" => "AccountNotSystemOwned: The given account is not owned by the system program",
    "3012" => "AccountNotInitialized: The program expected this account to be already initialized",
    "3013" => "AccountNotProgramData: The given account is not a program data account",
    "3014" => "AccountNotAssociatedTokenAccount: The given account is not the associated token account",

    // State.
    "4000" => "StateInvalidAddress: The given state account does not have the correct address",

    // Used for APIs that shouldn't be used anymore.
    "5000" => "Deprecated: The API being used is deprecated and should no longer be used",
};