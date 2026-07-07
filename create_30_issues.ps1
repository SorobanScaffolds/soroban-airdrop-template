$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
$repo = "SorobanScaffolds/soroban-airdrop-template"

$issues = @(
    "Day 1: Setup ESLint and Prettier for strict frontend formatting",
    "Day 2: Configure Tailwind CSS global theme and custom colors",
    "Day 3: Create reusable Button and Input UI components",
    "Day 4: Implement application Header and Navigation components",
    "Day 5: Scaffold the Hero Section for the Airdrop Landing Page",
    "Day 6: Set up Zustand global state store for user session",
    "Day 7: Implement Freighter Wallet Connection Hook",
    "Day 8: Build the Wallet Connect UI modal",
    "Day 9: Initialize Soroban smart contract with basic types",
    "Day 10: Implement smart contract Admin authentication",
    "Day 11: Write unit tests for contract Admin functions",
    "Day 12: Implement Merkle Root storage in the smart contract",
    "Day 13: Write unit tests for Merkle Root updates",
    "Day 14: Implement the core claim() function in Rust",
    "Day 15: Add double-claim prevention logic to claim()",
    "Day 16: Write comprehensive unit tests for the claim() function",
    "Day 17: Create Node.js script to generate Merkle Tree from JSON",
    "Day 18: Create API Route to serve Merkle Proofs to frontend",
    "Day 19: Build the Check Eligibility UI component",
    "Day 20: Integrate Eligibility UI with the Merkle API Route",
    "Day 21: Scaffold the Claim Success and Error UI states",
    "Day 22: Implement Soroban Transaction Builder in frontend",
    "Day 23: Integrate Freighter signing with the Transaction Builder",
    "Day 24: Handle Soroban RPC submission and polling",
    "Day 25: Display Transaction Hash and Explorer Link on success",
    "Day 26: Add smart contract deployment bash scripts",
    "Day 27: Implement Admin Dashboard for updating Merkle Roots",
    "Day 28: Add SEO metadata and Open Graph images to Next.js",
    "Day 29: Final end-to-end integration testing on Testnet",
    "Day 30: Write comprehensive documentation for mainnet launch"
)

$i = 1
foreach ($title in $issues) {
    Write-Host "Creating issue $i/30: $title"
    gh issue create --repo $repo --title $title --body "This is a daily task to build out the Soroban Airdrop Claim Template. This issue is part of our 30-day Drips Wave roadmap. Please submit a PR referencing this issue to close it." --label "wave-eligible"
    Start-Sleep -Seconds 2
    $i++
}
