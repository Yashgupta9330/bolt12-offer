export async function createOffer(username: string, expiry?: number) {
    try {
      const params = new URLSearchParams({ username });
      if (expiry) params.append('expiry', expiry.toString());
  
      const res = await fetch(`http://localhost:8081/api/create-offer?${params.toString()}`);
      return await res.json();
    } catch (e: any) {
      return {
        success: false,
        message: 'Failed to connect to backend: ' + e.message,
      };
    }
  }
  