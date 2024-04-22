# Sui Single-Writer-Friendly (SWF) Apps

> [!NOTE]
> This document is a copy of [sui/doc/src/learn/single-writer-apps.md](
> https://github.com/MystenLabs/sui/blob/21dad3ec1f2caf03ac4310e8e033fd6987c392bf/doc/src/learn/single-writer-apps.md)
> saved only for archival purposes.
> - Credit: [MystenLabs/sui](https://github.com/MystenLabs/sui/tree/main)
> - [Link to the license](https://github.com/MystenLabs/sui/blob/main/LICENSE-docs)
> - Changes: no significant changes have been made to the original content
> except formatting the source markdown file and changing some (relative) 
> links to sections to (absolute) URLs.

---

This page lists applications that can work in the single-writer model defined 
as [simple transactions](
https://github.com/MystenLabs/sui/blob/21dad3ec1f2caf03ac4310e8e033fd6987c392bf/doc/src/learn/how-sui-works.md#transactions-on-single-owner-objects) in Sui. Apart from the obvious single-writer friendly 
applications (such as simple peer to peer asset transfer), note that some 
proposals that typically require shared objects have been transformed to 
variants that require only a shared object as a final step and not for every 
action, such as voting and lotteries and DeFi Oracle price quotes submission.

1. Regular peer-to-peer (p2p) transactions ([see how to create a new Coin with 
just 7 lines of Sui Move code](
https://www.linkedin.com/posts/chalkiaskostas_startup-smartcontract-cryptocurrency-activity-6946006856528003072-CvI0)).
2. Confidential p2p Txs: same as FastPay but with pedersen commitments to hide 
amounts transferred; this still ensures input amount = output amount - we can 
set amount limits, i.e., N transfers up to $1,000 can be confidential.
3. Public bulletin board; users store only publicly accessed data, files, 
links, metadata.
4. Proof of existence: similar to the above, but for time-stamped documents; 
it can be extended to support commitment proof of existence, i.e. publish your 
hash, then reveal.
5. Private decentralized repository (users store private files, encrypted 
under their public keys; users' public keys can be represented as NFTs.
6. Extend the above for selected disclosure CV (resume) repository, University 
degrees repository.
7. Decentralized or conventional Certificate Authority. Authorities publish 
their signatures over certs, they can revoke any time (easier revocation).
8. Messaging service: apps, Oracles and Internet of Things (IoTs) exchanging 
messages. Sui is probably the best platform for any messaging protocol, as 
typically each response and message can be encoded with a single-writer NFT.
9. Extend the above to social networks; note that each post is a single-writer 
NFT. See a [smart contract implementation of a fully functional decentralized 
Twitter with just 50 lines of Sui Move code](
https://github.com/MystenLabs/sui/blob/main/sui_programmability/examples/nfts/sources/chat.move).
10. Extend the above to private messaging (i.e., decentralized WhatsApp or 
Signal).
11. Extend the above for any website / blog / rating platform (i.e., Yelp and 
Tripadvisor).
12. Personal GitHub, Overleaf LaTex editor, wish/shopping lists, etc.
13. Personal password manager.
14. Non-interactive games (i.e., advertise/evolve your SimCity, FarmVille 
state etc.).
15. Human vs. Computer games (i.e., chess AI that is programmed into the 
smart contract. The AI automatically plays back in the same transaction of 
user's chess move).
16. Coupons and tickets.
17. Mass minting of game assets.
18. Optimistic decentralized lottery: a new variant which needs only shared 
objects to declare winner but not to buy tickets; thus only one out of the 
million flows needs consensus.
19. Same for voting (each vote is an NFT) - only the aggregation part at the 
end needs to support fraud proofs with shared objects or have this happen at 
the application layer.
20. Same for most auction types (each bid is an NFT) - declaring a winner can 
be challenged by fraud proofs; thus, it’s the only step that requires a shared 
object.
21. Timed-release encrypted messages, including decrypting gift cards in the
future.
22. Posting price quotes (i.e., from Oracles, Pyth, etc.) can be 
*single-writer*, and a DEX trade can utilize shared objects. So Oracles can 
100% work on the single-writer model.
23. Job listing and related applications (i.e., a decentralized Workable).
24. Real estate contract repository: for tracking purposes only - payment is 
offline, otherwise it would be an atomic swap.
