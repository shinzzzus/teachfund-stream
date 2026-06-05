import { prisma } from "./prisma.js";

await prisma.paymentRecord.upsert({
  where: { externalId: "demo-teachfund-stream" },
  update: {},
  create: {
    externalId: "demo-teachfund-stream",
    owner: "GDEMO",
    target: 1000000,
    status: "seeded",
    projectName: "TeachFund Stream",
  },
});

console.log("Seeded TeachFund Stream");
await prisma.$disconnect();
