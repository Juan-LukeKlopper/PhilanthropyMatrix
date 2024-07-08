# PhilanthropyMatrix

PhilanthropyMatrix is a decentralized application (dApp) designed to facilitate secure and transparent charity donations using NFTs on the Secret Network. Users can propose and manage donations, mint NFTs, and securely transfer funds, ensuring privacy and security throughout the process. The system is versatile and can be adapted to other industries, providing a robust platform for internal approvals, rejections, and administrative management.

## Key Features

- **Propose Donations**: Users can submit donation proposals, approved by group admins.
- **Mint NFTs**: Approved donations can be turned into NFTs, representing the charitable contribution.
- **Secure Transactions**: Transactions are executed on the Secret Network, ensuring privacy and security.
- **User Management**: Admins can manage group memberships and approve donation proposals.
- **Versatility**: The system can be customized to accommodate other industries, providing internal approval and administrative control for various cases.

## Usage

1. *Propose a Charity*: Create a charity with all the required fields supplied and wait for approval.
2. *Upon approval*: Once the charity has been approved by a superuser, the admin (the person who proposed the charity group) can add other members.
3. *Propose a Donation*: Users can submit donation proposals, including details like name, cost, description, and image URL.
4. *Approve Donations*: Admins can approve or reject donation proposals.
5. *Mint NFTs*: Once approved, donations can be minted into NFTs.
6. *Secure Transactions*: Use the Keplr wallet to interact with the dApp and execute secure transactions.

## Development Deep Dive

### Architecture Overview

The platform uses PostgreSQL for data storage and management. It consists of:

- **Frontend**: Built with SvelteKit, providing an intuitive and responsive user interface ensuring a seamless user experience.
- **Backend**: Developed in Rust, handling API requests and interacting with the PostgreSQL database for its safety and concurrency features, ensuring robust and reliable performance.
- **Smart Contracts**: Deployed on the Secret Network to manage NFTs and secure transactions. The system is designed to be adaptable, supporting both web3 and web2 users.

### Setup Instructions

1. **Clone the Repository**:

    ```bash
    git clone https://github.com/yourusername/PhilanthropyMatrix.git
    cd PhilanthropyMatrix
    ```

2. **Install Dependencies**:

    - Frontend:

        ```bash
        cd frontend
        npm install
        ```

    - Backend:

        if you are using Linux/MacOS
        ```bash
        curl -sSfL https://www.shuttle.rs/install | bash
        ```

        on windows run
        ```bash
        iwr https://www.shuttle.rs/install-win | iex
        ```

        Alternatively install it using cargo
        ```bash
        cargo install cargo-shuttle
        ```

        After installing, log in with
        ```bash
        cargo shuttle login
        ```

3. **Set up the .env files**:

    Copy the example .env into the right location 
    ```bash
    cp frontend/.env-example frontend/.env
    ```

    This is what the .env contains

    ```.env
    VITE_BACKEND_URL=...
    VITE_SNIP721_CODE_ID=...
    VITE_SNIP721_CODE_HASH=...
    VITE_CHAIN_ID=...
    VITE_CHAIN_URL=...
    ```

    To connect you own instance of the backend and database you can update the `frontend/.env` file with the url for your shuttle project, They provision the database alongside the backend. if left unaltered you will connect to my backend instance and DB.



4. **Run the Application**:

    - Frontend:

        ```bash
        npm run dev
        ```

    - Backend:

        ```bash
        cd backend
        cargo shuttle project start
        cargo shuttle deploy
        ```

## Future Work

- **Enhanced User Profiles**: Improve user profile management and customization.
- **Donation Tracking**: Implement detailed tracking of donation usage and impact.
- **Expanded NFT Functionality**: Explore additional use cases for NFTs in charitable activities.
- **Cross-Industry Application**: Further develop the platform to support a variety of industries requiring internal approval and administrative management.

## Investor Pitch

### Problem Statement

Charity organizations and donors often face challenges related to transparency, security, and privacy in the donation process. Additionally, many web2 platforms lack robust mechanisms for internal approval and administrative management.

### Solution

PhilanthropyMatrix leverages blockchain technology to provide a secure, transparent, and privacy-focused platform for charity donations. Using NFTs on the Secret Network, the platform ensures that donations are securely managed and tracked, enhancing trust and accountability. The system is designed to be versatile, allowing for customization to suit various industries, enabling robust internal management and approval processes.

### Market Fit

The target audience includes charity organizations, donors, and blockchain enthusiasts who value transparency, security, and privacy in their transactions. The platform also caters to industries requiring internal approval mechanisms, broadening its market appeal. By supporting both web3 and web2 users, PhilanthropyMatrix provides a comprehensive solution for a wide range of applications.

### Potential Impact

PhilanthropyMatrix aims to revolutionize the charity donation process by providing a secure and transparent platform. This can attract more donors, increase trust in charitable organizations, and ensure that funds are used effectively for their intended purposes. The system's versatility allows it to be applied to other industries, enhancing internal management and administrative capabilities across various sectors.

### Additional Information

For HackSecret 3 Judges only, please review the `frontend/.env-example` file to find users created to give you access to some more functionality on the system.