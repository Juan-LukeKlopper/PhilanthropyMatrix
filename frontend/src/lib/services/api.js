import { SecretNetworkClient } from 'secretjs';
const chainId = import.meta.env.VITE_CHAIN_ID;
const API_URL = import.meta.env.VITE_CHAIN_URL;

export async function loginWithKeplr() {
  if (!window.keplr) {
    throw new Error("Please install Keplr extension");
  }

  await window.keplr.enable(chainId);
  const keplrOfflineSigner = window.getOfflineSigner(chainId);
  const [{ address }] = await keplrOfflineSigner.getAccounts();

  const signMessage = "Login request to my application";
  const signed = await window.keplr.signArbitrary(
    chainId,
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
  await window.keplr.enable(chainId);
  const keplrOfflineSigner = window.getOfflineSigner(chainId);
  const [{ address }] = await keplrOfflineSigner.getAccounts();

  const secretjs = new SecretNetworkClient({
    url: API_URL,
    chainId: chainId,
    wallet: keplrOfflineSigner,
    walletAddress: address,
  });

  return secretjs;
}

export async function changeCredentials(keplr_address, new_username, new_password) {
  const token = localStorage.getItem("token")
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/change-credentials`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
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

  await window.keplr.enable(chainId);
  const keplrOfflineSigner = window.getOfflineSigner(chainId);
  const [{ address }] = await keplrOfflineSigner.getAccounts();

  const signMessage = "Login request to my application";
  const signed = await window.keplr.signArbitrary(
    chainId,
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
  localStorage.setItem('id', responseData.user_id);
  localStorage.setItem('groups', responseData.groups);
  return responseData;
}

export async function getProfile() {
  const token = localStorage.getItem("token")
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

export async function addGroup(group) {
  const token = localStorage.getItem('token');
  const id = localStorage.getItem('id');
  let new_group = group;
  new_group.user_id = id;
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/groups/proposal/add`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(new_group)
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`Failed to add group: ${errorText}`);
  }

  return response.json();
}

export async function approveGroupProposalById(id) {
  const token = localStorage.getItem('token');
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/groups/approve`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({group_id: id})
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`Failed to add group: ${errorText}`);
  }

  return response.json();
}

export async function rejectGroupProposalById(id) {
  const token = localStorage.getItem('token');
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/groups/reject`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({group_id: id})
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`Failed to reject group: ${errorText}`);
  }

  return response.json();
}

export async function removeGroupById(id) {
  const token = localStorage.getItem('token');
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/groups/remove`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({group_id: id})
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`Failed to remove group: ${errorText}`);
  }

  return response.json();
}


export async function getAllGroups() {
  const token = localStorage.getItem('token');
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/groups/all`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`Failed to load groups: ${errorText}`);
  }

  return response.json();
}

export async function getGroupById(id) {
  const token = localStorage.getItem('token');
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/groups/${id}`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`Failed to fetch group: ${errorText}`);
  }

  return response.json();
}



export async function getAllGroupProposals() {
  const token = localStorage.getItem('token');
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/groups/proposals`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`Failed to load groups: ${errorText}`);
  }

  return response.json();
}

export async function getProposedGroupById(id) {
  const token = localStorage.getItem('token');
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/groups/proposal/${id}`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`Failed to fetch group: ${errorText}`);
  }

  console.log(response)

  return response.json();
}

export async function addMember(group_id, user_id, is_admin) {
  const token = localStorage.getItem('token');
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/groups/add_member`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`
    },
    body: JSON.stringify({ group_id, user_id, is_admin })
  });

  if (!response.ok) {
    throw new Error('Failed to add member');
  }

  return response.json();
}

export async function listGroupMembers(group_id) {
  const token = localStorage.getItem('token');
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/groups/members/${group_id}`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });

  if (!response.ok) {
    throw new Error('Failed to list group members');
  }

  return response.json();
}

export async function removeMember(group_id, user_id) {
  const token = localStorage.getItem('token');
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/groups/remove_member`, {
    method: 'DELETE',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`
    },
    body: JSON.stringify({ group_id, user_id })
  });

  if (!response.ok) {
    throw new Error('Failed to remove member');
  }

  return response.json();
}

export async function getDonationsForGroup(groupId) {
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/donations/list/${groupId}`, {
      method: 'GET',
  });
  if (!response.ok) {
      throw new Error('Failed to fetch donations for group');
  }
  return response.json();
}

export async function getDonationProposalsForGroup(groupId) {
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/donations/${groupId}/list/proposals`, {
      method: 'GET',
  });
  if (!response.ok) {
      throw new Error('Failed to fetch donation proposals for group');
  }
  return response.json();
}

export async function getDonationById(id) {
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/donations/${id}`, {
      method: 'GET',
  });
  if (!response.ok) {
      throw new Error('Failed to fetch donation details');
  }
  return response.json();
}

export async function getDonationProposalById(id) {
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/donations/proposal/${id}`, {
      method: 'GET',
  });
  if (!response.ok) {
      throw new Error('Failed to fetch donation details');
  }
  return response.json();
}

export async function proposeDonation(proposal) {
  const token = localStorage.getItem('token');
  console.log(proposal)
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/donations/proposal/add`, {
      method: 'POST',
      headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
      },
      body: JSON.stringify(proposal)
  });
  if (!response.ok) {
      throw new Error('Failed to propose donation');
  }
  return response.json();
}

export async function approveDonation(id, group_id) {
  const token = localStorage.getItem('token');
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/donations/approve`, {
      method: 'POST',
      headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
      },
      body: JSON.stringify({ proposal_id: id, group_id: group_id })
  });
  if (!response.ok) {
      throw new Error('Failed to approve donation');
  }
  return response.json();
}

export async function rejectDonation(id, group_id) {
  const token = localStorage.getItem('token');
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/donations/reject`, {
      method: 'POST',
      headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
      },
      body: JSON.stringify({ proposal_id: id, group_id: group_id })
  });
  if (!response.ok) {
      throw new Error('Failed to approve donation');
  }
  return response.json();
}

export async function removeDonation(id, group_id) {
  const token = localStorage.getItem('token');
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/donations/remove`, {
      method: 'POST',
      headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
      },
      body: JSON.stringify({ proposal_id: id, group_id: group_id })
  });
  if (!response.ok) {
      throw new Error('Failed to approve donation');
  }
  return response.json();
}

export async function linkDonationContract(groupId, donationId, contractAddress) {
  const token = localStorage.getItem("token");

  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/donations/link_address`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`
    },
    body: JSON.stringify({ group_id: groupId, donation_id: donationId, contract_address: contractAddress })
  });

  if (!response.ok) {
    throw new Error('Failed to link donation contract');
  }

  return response.json();
}

export async function getDonations() {
  const response = await fetch(`${import.meta.env.VITE_BACKEND_URL}/donations/active`, {
      method: 'GET',
  });
  if (!response.ok) {
      throw new Error('Failed to fetch donations for group');
  }
  return response.json();
}
