'use client';

import { createOffer } from '@/lib/api';
import { useState } from 'react';


export default function OfferForm() {
  const [username, setUsername] = useState('');
  const [expiry, setExpiry] = useState('');
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<null | {
    success: boolean;
    message: string;
    offer?: string;
    dns_status?: string;
  }>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setResult(null);

    const response = await createOffer(username, expiry ? parseInt(expiry) : undefined);
    setResult(response);
    setLoading(false);
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <input
        type="text"
        placeholder="Username (e.g. yashg)"
        className="w-full border rounded p-2"
        value={username}
        onChange={(e) => setUsername(e.target.value)}
        required
      />
      <input
        type="number"
        placeholder="Expiry (seconds, optional)"
        className="w-full border rounded p-2"
        value={expiry}
        onChange={(e) => setExpiry(e.target.value)}
      />
      <button
        type="submit"
        className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 disabled:opacity-50"
        disabled={loading}
      >
        {loading ? 'Generating...' : 'Create Offer'}
      </button>

      {result && (
        <div className="mt-4 p-4 bg-gray-100 rounded border">
          {result.success ? (
            <>
              <p className="font-semibold text-green-700">{result.message}</p>
              <p className="break-words mt-2"><strong>Offer:</strong> {result.offer}</p>
              <p className="text-sm text-gray-600 mt-2">{result.dns_status}</p>
            </>
          ) : (
            <p className="text-red-600">{result.message}</p>
          )}
        </div>
      )}
    </form>
  );
}
``
