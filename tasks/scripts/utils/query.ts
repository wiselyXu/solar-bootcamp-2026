
import { getAccount, getMint, Mint } from "@solana/spl-token";
import { Account, Connection, PublicKey } from "@solana/web3.js";
import dotenv from "dotenv";

dotenv.config();

async function main() {
  //  queryATAInfo('4f7BVSB2KeGJYKzEvPfCxRZPtDKN8Cvn6cZqReT99PLk', new Connection(process.env.RPC_URL!, 'confirmed'));
  queryMintInfo('3wGm7vi71y3iQwJZbuKMuJTYX9GC8nyjawr9ZW6MerHw', new Connection(process.env.RPC_URL!, 'confirmed'));
}

main().catch(console.error);

async function queryATAInfo(ataStr: String, connection: Connection) {
    const ata = new PublicKey(ataStr);
    const ataInfo = await getAccount(connection, ata);

    console.log(`ATA Address: ${ata.toBase58()}`);
    console.log(`ATA Balance: ${ataInfo.amount / BigInt(10 ** 6)} tokens`);
    console.log(`ATA Owner: ${ataInfo.owner.toBase58()}`);
    console.log(`ATA Mint: ${ataInfo.mint.toBase58()}`);

    return ataInfo;
    
    
}

async function queryMintInfo(mintAccountStr: String, connection: Connection)  : Promise<Mint>{
    const mintAccountPublicKey = new PublicKey(mintAccountStr);
    
    const mintInfo =  getMint(connection, mintAccountPublicKey);
    const decimals = (await mintInfo).decimals;
    console.log(`Mint Address: ${mintAccountPublicKey.toBase58()}`);
    console.log(`Mint Supply: ${(await mintInfo).supply / BigInt(10 ** decimals)} tokens`);
    console.log(`Mint Decimals: ${decimals}`);
    return mintInfo;
    
    
}

