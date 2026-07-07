$repo = "https://github.com/SorobanScaffolds/soroban-airdrop-template/issues/new"

$issues = @(
    @{ title="Implement claim function with Merkle Proof Verification"; labels="wave-eligible,smart-contract,rust"; body="We need a robust claim function in our Soroban smart contract. The function must accept:`n1. The claimant's address (Address).`n2. The amount of tokens they are eligible for (i128).`n3. The Merkle Proof (Vec<BytesN<32>>) that verifies they are part of the Merkle Root stored on the contract." },
    @{ title="Create Contract Tests for Airdrop Initialization"; labels="wave-eligible,smart-contract,testing"; body="We need to write robust tests using the soroban-sdk test framework to ensure the contract initializes correctly.`nThe initialization should set the Admin Address, Token Address, and the initial Merkle Root." },
    @{ title="Implement update_merkle_root admin function"; labels="wave-eligible,good first issue,smart-contract"; body="Create a function update_merkle_root(admin: Address, new_root: BytesN<32>). It should verify that the caller is the admin before updating the root in the contract's storage." },
    @{ title="Scaffold the Landing Page UI"; labels="wave-eligible,frontend,ui/ux"; body="Create a beautiful landing page (frontend/src/app/page.tsx) for the Airdrop claim site.`nIt should have:`n1. A hero section with a title 'Claim Your Airdrop'.`n2. A description of the project.`n3. A large CTA button that will eventually link to the Freighter wallet connection.`nPlease use Tailwind CSS and keep the design sleek and modern (dark mode preferred)." },
    @{ title="Integrate Freighter Wallet Connection Hook"; labels="wave-eligible,frontend,web3"; body="We need to connect users to the Stellar network using the Freighter wallet.`nCreate a custom React hook useFreighter() in frontend/src/hooks/ that utilizes @stellar/freighter-api to:`n- Check if Freighter is installed.`n- Request connection and return the user's public key.`n- Provide a disconnect function." },
    @{ title="Build the 'Check Eligibility' Component"; labels="wave-eligible,frontend,react"; body="Create a component EligibilityChecker.tsx.`nIt should feature an input field where a user can paste their Stellar public key (or use their connected Freighter wallet).`nUpon clicking 'Check', it should (for now) display a mock loading state, then show a success message 'You are eligible for 1000 TOKENS' or an error 'Address not eligible'." },
    @{ title="Create the Airdrop Claim Transaction Builder"; labels="wave-eligible,frontend,soroban"; body="Using the stellar-sdk and soroban-client, write a utility function in frontend/src/utils/soroban.ts that builds the Soroban transaction to call the claim function on our smart contract. It should accept the claimant address, amount, and the Merkle proof array." },
    @{ title="Setup Global State (Zustand or Context API)"; labels="wave-eligible,good first issue,frontend"; body="Set up a simple global state to track:`n- Is the wallet connected?`n- What is the connected address?`n- What is the user's eligibility status?`nWe recommend using zustand for this, but standard React Context is also fine." },
    @{ title="Create Merkle Tree Generator Script"; labels="wave-eligible,backend,scripting"; body="We need a Node.js script that takes a JSON file of eligible users (e.g., {'GABC...': 100, 'GXYZ...': 500}) and generates a Merkle Tree.`nYou can use merkletreejs and keccak256.`nThe script should output the Merkle Root and a JSON file containing the proofs for every user, which the frontend will consume." },
    @{ title="Create a script to deploy the Airdrop Contract"; labels="wave-eligible,backend,stellar-cli"; body="Write a bash or Node script in the contracts/ directory that uses the stellar CLI to build the WASM and deploy it to the Soroban Testnet. It should automatically log the generated Contract ID to a file so the frontend can read it." }
)

Add-Type -AssemblyName System.Web

foreach ($issue in $issues) {
    $t = [System.Web.HttpUtility]::UrlEncode($issue.title)
    $l = [System.Web.HttpUtility]::UrlEncode($issue.labels)
    $b = [System.Web.HttpUtility]::UrlEncode($issue.body)
    $url = "$repo`?title=$t&labels=$l&body=$b"
    Start-Process $url
    Start-Sleep -Milliseconds 500
}
