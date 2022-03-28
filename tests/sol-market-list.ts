import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { SolMarketList } from '../target/types/sol_market_list';

describe('sol-market-list', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SolMarketList as Program<SolMarketList>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
