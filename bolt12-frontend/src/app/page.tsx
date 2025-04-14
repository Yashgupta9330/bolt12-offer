import OfferForm from "../components/OfferForm";


export default function Home() {
  return (
    <main className="min-h-screen bg-gray-100 p-6 flex items-center justify-center">
      <div className="bg-white shadow-xl rounded-2xl p-8 w-full max-w-xl">
        <h1 className="text-2xl font-bold mb-4">⚡ BOLT 12 Offer Generator</h1>
        <OfferForm />
      </div>
    </main>
  );
}
