import { SecretNetworkClient } from 'secretjs';

export async function loginWithKeplr() {
  if (!window.keplr) {
    throw new Error("Please install Keplr extension");
  }

  await window.keplr.enable("pulsar-3");
  const keplrOfflineSigner = window.getOfflineSigner("pulsar-3");
  const [{ address }] = await keplrOfflineSigner.getAccounts();

  const signMessage = "Login request to my application";
  const signed = await window.keplr.signArbitrary(
    "pulsar-3",
    address,
    signMessage
  );

  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/keplr-login`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ address, pubkey: signed.pub_key.value, sign_message: signMessage, signature: signed.signature })
  });

  if (!response.ok) {
    const errorText = await response.text();
    console.error('Keplr login failed:', errorText);
    throw new Error('Keplr login failed');
  }

  const responseData = await response.json();
  localStorage.setItem('token', responseData.token);

  return responseData;
}

export async function createSecretNetworkClient() {
  await window.keplr.enable("pulsar-3");
  const keplrOfflineSigner = window.getOfflineSigner("pulsar-3");
  const [{ address }] = await keplrOfflineSigner.getAccounts();

  const secretjs = new SecretNetworkClient({
    url: "https://api.pulsar.scrttestnet.com",
    chainId: "pulsar-3",
    wallet: keplrOfflineSigner,
    walletAddress: address,
  });

  return secretjs;
}

export async function changeCredentials(keplr_address, new_username, new_password) {
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/change-credentials`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ keplr_address, new_username, new_password })
  });

  if (!response.ok) {
    const errorText = await response.text();
    console.error('Failed to change credentials:', errorText);
    throw new Error('Failed to change credentials');
  }

  return response.json();
}

export async function addWallet() {
  if (!window.keplr) {
    throw new Error("Please install Keplr extension");
  }

  await window.keplr.enable("pulsar-3");
  const keplrOfflineSigner = window.getOfflineSigner("pulsar-3");
  const [{ address }] = await keplrOfflineSigner.getAccounts();

  const signMessage = "Login request to my application";
  const signed = await window.keplr.signArbitrary(
    "pulsar-3",
    address,
    signMessage
  );

  const username = "temp"
  const password = "temp"

  const token = localStorage.getItem("token")

  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/add-wallet`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`
    },
    body: JSON.stringify({ username, password, keplr_address: address, pubkey: signed.pub_key.value, sign_message: signMessage, signature: signed.signature})
  });

  if (!response.ok) {
    throw new Error('Failed to add wallet address');
  }

  return response.json();
}

export async function register(user) {
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/register`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(user)
  });

  if (!response.ok) {
    const errorText = await response.text();
    console.error('Registration failed:', errorText);
    throw new Error('Registration failed');
  }

  return response.json();
}

export async function login(user) {
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/login`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(user)
  });

  if (!response.ok) {
    const errorText = await response.text();
    console.error('Login failed:', errorText);
    throw new Error('Login failed');
  }

  const responseData = await response.json();
  localStorage.setItem('token', responseData.token);
  return responseData;
}

export async function getProfile(token) {
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/profile`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });

  if (!response.ok) {
    const errorText = await response.text();
    console.error('Get profile failed:', errorText);
    throw new Error('Get profile failed');
  }

  return response.json();
}
